# Agent Research 08 — T3 Code v0.1.10 Deep Source Audit

*Refresh + deepen of `PLAN.md` §19 (audited `d5b921ce` / v0.1.3, 2026-10-25). New audit: 2026-11-17, against tag `v0.1.10` @ commit `7d32075e3757ded707914e2450c3e392a9665765` (2026-11-16 12:13:35 -0800), shallow-cloned to `C:\Users\ASUS\AppData\Local\Temp\opencode\t3code`. All line refs below are from that commit unless noted. This report reads actual source; it does not repeat the §19 summary.*

---

## 1. Freshness

| Signal | Value (fetched 2026-11-17) |
|---|---|
| Repo | `github.com/pingdotgg/t3code` — "The best way to control Claude Code, Codex and more from any computer or phone" (`t3.codes`) |
| License | **MIT** (unchanged) |
| Stars / forks / watchers | **63,874** / **11,271** / 63,874 (§19 era was far smaller) |
| Open issues | 3,493 |
| Pushed at | 2026-11-17T18:14:51Z (active *today*) |
| Created | 2026-02-08 (≈9 months old) |
| Default branch | `main`; language TypeScript; repo size ≈870 MB |
| Latest release | **v0.1.10**, tag target `7d32075e…`, created 2026-11-16T20:13:35Z, published **2026-11-17T01:44:04Z**, not prerelease — "**1,924 changes from 360 contributors**" |
| v0.1.10 headline changes | **Windows service + remote update** (run as service, update without manual restart), T3 Connect **device approval**, keep-computer-awake while agents work, native thread resume, reply-to-response, themes/background images, 9 settings languages |
| Release artifacts | win32 x64/arm64 `.zip` + `.exe`, `.deb`, `.dmg`, `.AppImage`, tarballs, per-platform `latest.yml`, **`SHA256SUMS`** |
| Source size (excl. vendored `.repos/`) | ≈**6,993** TS/TSX files, ≈**2,172** test files (§19 said ~5,300 / 1,482) |
| `Orchestrator.ts` | **16,160 lines** (§19 said 12,692 — **+27% growth in ~3 weeks**) |

**Freshness verdict:** extremely fresh and fast-moving; the §19 claims about *mechanisms* remain broadly true, but the *numbers* (file counts, Orchestrator size, terminal/live-stream caps) have changed and there is a whole new service/update lifecycle. Treat source refs here as the current truth.

---

## 2. Architecture (what it actually is, confirmed at v0.1.10)

Still **an agent-harness control surface, not an agent**. An Effect-TS Node server wraps provider CLIs (Claude Code, Codex, Cursor, OpenCode, Antigravity, Grok Build, Muse, Pi) as subprocesses over ACP; React web / Electron desktop / React Native mobile clients connect over authenticated WebSocket RPC. T3 owns orchestration state, checkpoints, auth, remote access (Tailscale / T3 Connect), scheduled + webhook-triggered runs, and now a Windows service launcher with remote self-update. The agent loop, tool suite, and permissions live inside the wrapped CLIs.

Layering relevant to adoption:

```
clients ──WS(RPC)──> auth (DPoP / PKCE / WS tickets)
                       │
                       └─> orchestration-v2/Orchestrator.ts  (command → events + effects)
                                    │ EventSink.ts  (ONE SQLite tx: append events + project + enqueue outbox)
                                    │ CommandReceiptStore.ts (idempotent command keys)
                                    └─> EffectOutbox ──drain──> EffectWorker.ts (lease/claim effects)
provider-acp/ ── policy ladder + canonical path containment ──> wrapped provider CLIs
serviceLauncher.ts + cloud/selfUpdate.ts ── remote update lifecycle (active/trial, DB backup/restore)
```

Important nuance vs §19: calling `Orchestrator.ts` a "pure decision service, no I/O" is **overstated**. `makeOrchestrator` (`Orchestrator.ts:1549`) yields `FileSystem`, provider adapters, `KeyedLock` (`messageIntake`, :1585), checkpoint/projection/effect-outbox services, etc. The accurate reusable *pattern* is "decide → emit domain events + outbox effects → commit atomically elsewhere," not literal purity, and definitely **not** the 16k-line monolith.

---

## 3. Top 10 mechanisms — file refs + what to copy + `Sx.y` adoption map

### 1. Transactional outbox: event + projection + outbox in one SQLite transaction
**Source:** `apps/server/src/orchestration-v2/EventSink.ts` (1,234 lines).
- `commitThenPublishNow` (`:316–341`): `sql.withTransaction(...)` wrapped in `Effect.uninterruptibleMask`; a `Semaphore(1)` **publish lane** acquired inside the transaction so publication can never be reordered (`:314–315`, `:330–340`). "Once it commits, every subscriber must hear about it before this writer can exit."
- `commitCommand` (`:532–599`): optimistic-concurrency guard reads the projection row (`status`, `activeAttemptId`, `:538–556`) → `normalizeEvents` (`:576`) → `eventStore.append` (`:581`) → `applyStoredEvents` / projection merge (`:585`) → `effectOutbox.enqueue` (`:586`) — **all inside the one transaction** — and only after commit does it `notifyAvailable` + publish (`:589–596`).
- `publishStoredEvents` (`:303–312`) records run outcomes *before* publish so an interrupted publish cannot lose a committed outcome.
- `guardUserInputCancellations` (`:345–387`) re-checks cancellations inside the write transaction.

**Copy:** event schema `{seq, stream_id, type, payload, causation_id, correlation_id, ts}`; single transaction for event+projection+outbox; commit-then-publish under a serialize lane; outcome-before-publish. **Do not** copy the monolith — keep modules small.
**Maps to:** `S2.5` (T3-style durable state), `S2.9` (durability contracts), `S0.4` (audit chain — ours adds hash chaining they lack).

### 2. Lease-based, single-consumer outbox worker with per-thread serialization
**Source:** `apps/server/src/orchestration-v2/EffectWorker.ts` (1,095 lines).
- `claimNext({workerId, ...})` (`:898`, `:911`) with durable leases; "a running effect blocks later work on its thread and its lease is never reclaimed in-process" (`:721`); only the claiming worker settles its row (`:755`).
- Lost-lease detection (`:955`, `:993`), `retryWhileLocked` with spaced retry (`:759–762`), cancellation re-check after claim (`:929`), notification + `availableAt` persistence + **liveness polling** for missed wakeups (`:1065–1092`), cool-off on failed claims (`:1086`).

**Copy:** singleton worker, lease + fencing, per-aggregate serialization, dead-letter poison events, liveness poll + notify. This is the operational half of §1.
**Maps to:** `S2.9` (single worker + poison events), `S2.10` (actors/mailboxes, lease + fencing tokens), `S5.3` (durable scheduler engine).

### 3. Idempotent command receipts (retries never duplicate effects)
**Source:** `apps/server/src/orchestration-v2/CommandReceiptStore.ts` (186 lines); persistence repo `apps/server/src/persistence/OrchestrationCommandReceipts.ts`.
- `CommandReceiptV2` / `ProjectCommandReceiptV2` schemas (`:43–63`): `{commandId, threadId|projectId, commandType, acceptedAt, resultSequence, status: accepted|rejected, error}`.
- `insertIfAbsent` (`:67–70`, `:140–149`) is the idempotency primitive; `upsert`, `getByCommandId`, `getProjectByCommandId` (`:71–81`); a `Semaphore`/HMAC-locked read is not needed because the DB unique index does the work.

**Copy:** `command_receipts` unique index, accept/reject + result sequence, "no effect without receipt" property test.
**Maps to:** `S2.5` ("kill mid-command → restart → receipt prevents duplicate effect"), `S2.9`, `S2.11` (property "no effect without receipt").

### 4. Four-tier mode ladder × sandbox × tool-kind → allow/ask/deny
**Sources:** `packages/contracts/src/providerPolicy.ts:25–43` and `packages/provider-acp/src/server/clientPolicy.ts` (228 lines).
- Ladder: `approval-required(0) < auto-accept-edits(1) < auto(2) < full-access(3)`; **`DEFAULT_RUNTIME_MODE = "full-access"`** (`providerPolicy.ts:32`, rank map `:34–39`, `isRuntimeModeAbove` `:41–43`).
- `acpOperationDisposition` (`clientPolicy.ts:143–186`): read kinds (`read|search|think`) never ask (`:82–98`); if approval required → `ask`; else sandbox switch: `readOnly` → deny, `workspaceWrite` → allow only if every `location` is inside canonical cwd/`writableRoots` (`:100–141`), `dangerFullAccess`/`externalSandbox` → allow, unknown → deny (**fail-closed default case** `:184`).
- `makeAcpClientPolicyGrants` (`:237–251`): approving an `execute` request grants execute permission only for that turn or session (ACP cannot bind commands to terminals).

**Copy:** the explicit rank/ceiling algebra and the fail-closed `default → deny`; **do not** copy "new threads default to full access" — flip to deny-by-default (`S0.4`).
**Maps to:** `S0.4` (tier pipeline, default `never`), `S1.3` (tiered permission UX), `S1.2` (rule resolution).

### 5. TOCTOU-safe recheck under the command lock + organization mode ceiling
**Sources:** `apps/server/src/orchestration-v2/DispatchModeLimit.ts` (101 lines), `apps/server/src/mcp/McpToolAccess.ts` (439 lines).
- `exceededDispatchModeLimit` (`DispatchModeLimit.ts:33–41`), `RuntimeModeCeiling` (`:43–47`), `narrowToCeiling` (`:50–58`) lowers any caller limit above the admin ceiling, and `runtimeModeCeilingRefusal` (`:73–94`) judges a planned command by its events (`thread.created`, `thread.runtime-mode-updated`, `run.created`, `run-attempt.created`) so every mode-changing command is covered.
- `McpToolAccess.writesThreads` (`McpToolAccess.ts:199–231`): builds a `Ref` for a `DispatchModeRefusal`, provides it as `DispatchModeLimit` to the handler, and on any error converts a recorded refusal into `runtime_mode_escalation_denied` / `interaction_mode_escalation_denied` (`:233–240`). Comment at `:217`: "Modes can rise after this check; the orchestrator rechecks under lock." `writesEnvironment` (`:325–354`) re-checks full-access after waits.

**Copy verbatim:** recheck authorization *after* awaits, under the same lock that commits the effect; record the refusal so the error explains "its user changed it while this call ran." This is the single most transferable TOCTOU pattern in the codebase.
**Maps to:** `S1.8` (capability isolation / guard evolution), `S0.4` (TOCTOU-safe decision pipeline), `S2.12` (zero standing privileges / scope narrowing), `S1.7` (consent grant scope/TTL/revoke).

### 6. Compile-time-mandatory tool access declarations + lint enforcement
**Sources:** `apps/server/src/mcp/McpToolAccess.ts`, `oxlint-plugin-t3code/rules/no-raw-mcp-registration.ts`, `oxlint-plugin-t3code/rules/no-rpc-permission-bypass.ts`, enabled in `vite.config.ts:312`.
- `Declaration` uses a **private constructor + unique-symbol token** (`McpToolAccess.ts:39–46`, `:87–114`) so only `McpToolAccess` can build tool declarations; `checkedHandler` / `Struct.lambda` extracts handlers; `routed` (`:415–462`) strips `environmentId` from handler params so it cannot leak into writes.
- Risk/access vocabulary is the declaration functions themselves: `reads` (`:144`), `readsAsCaller` (`:150`), `actsAsCaller` (`:153`), `writes` (`:168`), `writesWithCapability` (`:189`), `writesThreads` (`:199`), `startsThreads` (`:254`), `either` (`:285`), `afterForwarding` (`:311`), `writesEnvironment` (`:325`).
- `refuseReadOnlyClient` (`:116–128`) blocks any environment-changing tool for a read-only-approved MCP client.
- `no-raw-mcp-registration.ts` bans importing/re-exporting `McpServer` from `effect/ai` anywhere but `McpHttpServer` and bans `.addTool/.addResource/.addPrompt/McpServer.toolkit` (`:6–30`, `:49–130`). `no-rpc-permission-bypass.ts` bans raw `rpc/protocol` imports, `RpcPermissionGuard` and `.client` access outside the shared boundary (`:21–89`), with reviewed per-file opt-outs.

**Copy:** make risk declarations *the only constructible type* (private ctor/brand) and add a lint rule that fails any code path registering a tool outside the guarded factory. Stronger than documentation-only capability claims.
**Maps to:** `S0.2` (uniform tool registry envelope), `S2.2` (plugin API + permission manifests), `S2.11` (fail-closed property tests), `S1.8` ("agent cannot rewrite its own tooling/crew").

### 7. Symlink-canonicalized, fail-closed path containment (NTFS-aware)
**Sources:** `packages/provider-acp/src/server/clientPolicy.ts:22–69` and `apps/server/src/workspace/WorkspaceFileSystem.ts` (607 lines).
- `acpCanonicalPathForContainment` (`clientPolicy.ts:41–69`): walks ancestors with `realpathSync.native`, resolving **existing** ancestors before `..` ("lexical normalization can hide traversal"), accumulates missing suffixes, fail-closed on any non-ENOENT error. `acpPathIsWithinRoot` (`:30–38`) does path-relative containment.
- `isGitMetadataName` (`WorkspaceFileSystem.ts:34–38`) uppercases, strips trailing dots/spaces and `:` stream suffix, and matches `.GIT`/`GIT~1` (NTFS alias tricks). `isInGitFolder` (`:235–251`) detects repos via `HEAD`+(`commondir`|`objects`+`refs`) and stops at linked-worktree `.git` files.
- `resolveWriteTarget` (`:496–575`) canonicalizes nearest existing ancestor, checks **both** the real path and the as-named path for git folders (`:562–566`), then writes with `O_NOFOLLOW | O_CREAT | O_TRUNC` (`:606–615`, "a symlink swapped in after the check is not followed"). `resolveReadTarget` (`:259–371`) uses `O_NONBLOCK` to avoid FIFO hangs and refuses network paths outside cwd to avoid NTLM sign-in (`:179–181`, `:267–275`). 1 MiB read cap (`:32`), fatal UTF-8 decode, BOM preservation (`:586–591`).

**Copy directly:** canonicalize-before-`..`, check named + resolved path, `O_NOFOLLOW` on write, NTFS alias normalization, `O_NONBLOCK`. **Caveat:** this protects *these specific APIs*; §19's "projects are organizational boundaries, not filesystem sandboxes" still holds for subprocesses.
**Maps to:** `S1.2` (normalize 8.3 names/UNC/symlinks/junctions, PATH hijack), `S1.1`, `S1.4` (egress/SSRF hygiene), `S2.13` (normalization at trust boundary).

### 8. Checkpoints as hidden git refs + restore-safety gating
**Sources:** `apps/server/src/orchestration-v2/CheckpointRestoreSafety.ts` (104 lines); used in `Orchestrator.ts:107–110`; hidden refs `refs/t3/checkpoints/*` via `CheckpointServiceV2`.
- `SHARED_WORKSPACE_RESTORE_MESSAGE` (`:9–10`) tells the user to rewind conversation without restoring files, or delete side chats.
- `isCheckpointRestoreIsolated` (`:13–105`): resolves cwd vs `thread.worktreePath` with `realPath` (tolerating cleanup removal), then scans all live + archived threads for overlapping cwd/worktree/project-root paths (`:49–101`), with a narrow exception for idle side chats that never touched files (`:52–65`) and coverage of shared provider-session cwds (`:71–79`). Safety checked **at admission and at rollback** (`:12`).

**Copy verbatim:** "a provider/agent that cannot roll back its conversation must reject *before* changing the filesystem," plus the shared-worktree overlap check. Our version: `refs/harness/checkpoints/*`.
**Maps to:** `S1.1` (git integration & checkpoints — already in the plan), `S2.10` (shared-scope serialization), `S2.9` (snapshot/durable state).

### 9. Infrastructure-level auth: DPoP + PKCE-only OAuth + scoped one-use WS tickets
**Sources:** `apps/server/src/auth/dpop.ts`, `apps/server/src/auth/McpOAuth.ts`, `apps/server/src/auth/EnvironmentAuth.ts`, `packages/contracts/src/auth.ts`.
- **DPoP:** `checkRequestDpopProof` (`dpop.ts:57–131`) verifies proof against method/URL/iat and expected thumbprint + access-token hash; replay defense stores a `sha256(thumbprint:jti)` marker in the secret store via create-if-absent (`:93–128`, `DPOP_REPLAY_MARKER_PREFIX` `:22`); `verifyRequestDpopProof` (`:133–139`) consumes it. Failure taxonomy at `:24–41`.
- **PKCE-only OAuth for MCP clients:** `code_challenge_method=S256` required (`McpOAuth.ts:405–412`), timing-safe verifier check (`:308–314`), self-signed `client_id` carrying name/redirect URIs (`:280–306`), redirect URIs restricted to https or localhost (`:341–347`), **no client secrets**, CSRF token bound to the DPoP key thumbprint (`:316–325`, `:432–438`), authorization code deleted before later checks (RFC 6749 §4.1.2, `:541–560`).
- **Scoped WS tickets:** `issueWebSocketTicket` (`EnvironmentAuth.ts:1442–1463`) mints a short-lived ticket with audience `use: "rpc" | "media"`; RPC tickets are bound to the proof key. Upgrade (`:1470–1500`) checks audience, requires DPoP proof in query (WS can't set headers), and consumes the jti once.

**Copy:** DPoP-bound short-lived tokens, PKCE S256-only, audience-scoped single-use tickets, jti replay store. Maps to our Ed25519/DPoP plan; note our `S2.12` also adds JWKS/rotation/revocation that T3 does not fully specify.
**Maps to:** `S2.12` (identity & IAM lifecycle), `S2.7`/`S0.7` (WS protocol handshake), `S5.3` (relay ingress).

### 10. Remote ingress secrets, bounded transports, and NEW Windows-service + remote-update lifecycle

**10a. One-use secret refs + HMAC webhooks + bounded streams/terminal**
- `apps/server/src/secrets/SecretRequests.ts`: refs are `secret-ref:<32 hex>` = HMAC(threadId\0turnItemId) keyed by a random per-server salt (`:33–38`, `:83–95`); 24h TTL (`:36`, `:252–255`); `consume` is serialized by a `Semaphore(1)` (`:224–234`) and `consumeRef` **deletes the stored value before handing it out** (`:259–270`) — "a ref is never used twice"; owner-only `0700`/`0600` files via `wx` + partial+rename (`:272–287`). Tool prompt in `apps/server/src/mcp/toolkits/orchestrator/tools.ts:211` asks for one secret per card and returns a one-use `secretRef`.
- `apps/server/src/scheduledTasks/webhookVerification.ts`: HMAC-SHA256 over raw body, hex/base64 + prefix handling, `constantTimeEquals` hashes both inputs before `timingSafeEqual` (`:11–47`). `webhookTemplate.ts`: `CREDENTIAL_NAME` redaction of headers/query (`:17–45`), mention-breaking + unpredictable per-request framing of untrusted webhook data (`:1`, `:52`, `:148+`); dedupe/rate-limit via `scheduled_task_webhook_pending` table in `ScheduledTaskService.ts:960`, `:1151–1156`, outcomes `rate_limited|queue_full` (`:101`, `:174–175`).
- **Bounded transports (caps updated vs §19):** `LiveStreamBudget.ts` — 1,000 items / **8 MiB** serialized (`:18–19`), WeakMap size cache (`:27–37`), ACK-based release (`:86–197`), `bufferLatestLiveStream` coalescing (`:244–318`), replay+live budget (`:333–387`). `packages/provider-acp/src/server/clientTerminals.ts:16–21` — 16 live / 64 unreleased / 32 retained terminals, 16 MiB retained / **8 MiB per-output** limit, truncate-from-start at UTF-8 boundary (`:51–65`). `packages/contracts/src/terminal.ts:6` — 512 KiB default buffer, ANSI-sequence-safe tail trim (`:104–131`), 65,536-char writes (`:194`), env ≤128 keys/8,192 bytes (`:141–147`); `packages/client-runtime/src/state/terminalOutput.ts:58–59` — 16 KiB chunks × 1,024. (§19's "5,000 lines / 8 MiB" is the older number; current code is byte-based, multi-layered.)

**Maps to:** `S8.6` (key & secret lifecycle — one-use refs), `S0.4` (secret redaction before persistence), `S5.3` (HMAC webhook ingress, dedupe/rate-limit), `S2.5`/`S1.4` (stream budgets, retention caps), `S2.13` (untrusted-data framing).

**10b. NEW Windows service + remote-update lifecycle**
- `apps/server/src/serviceLauncher.ts` (1,529 lines): standalone launcher with **active/trial** child roles (`:64–65`), versioned runtimes with `.install-complete` sentinel (`:73–87`, `:689–700`), Windows Task Scheduler registration via `schtasks` + per-account task name (`:101–129`, `:753–754`). Durable state (`service.json`) written with atomic `wx`+fsync+rename `replaceFileDurably` (`:663–687`); selection under `acquireServiceSelectionLock`.
- Update flow: active server requests update → `#beginTrial` does an **online DB backup first**, then terminates active and starts the trial (`:1297–1313`); `#handlePrepared` **commits durably** (`activeVersion = targetVersion`, status `committed`) before discarding the backup and repointing the service unit under a lease (`:1315–1344`, `:1347–1383`). Failure paths: `#handlePreparedTimeout` (`:1385–1392`), trial exit → `#returnToPrevious` (`:1394–1416`, `:1488–1514`) restores the DB (`.restore-pending` marker `:264`, `:575–586`; symlink-aware `restoreTarget` `:589–609`; `restoreDatabaseBackup` `:611–639`), writes `rolled-back`/`failed`, restarts the old version. Exit code 78 = "state dir owned by another server" → idle/reclaim loop (`:60–62`, `:1438–1486`). Online backup space preflight refuses updates on full disks (`:437–498`).
- `apps/server/src/cloud/servicePreflight.ts:24–78`: blocks a version whose launcher protocol is unsupported or whose DB has **newer migrations** (downgrade protection). `cloud/selfUpdate.ts` (`:55–78`, `:52–53`) orchestrates; `cloud/pinnedRuntime.ts:217–227`, `:262–284` verifies the downloaded runtime archive against `SHA256SUMS` with SHA-256.

**Copy:** active/trial blue-green service update, DB backup-before-stop, durable commit marker before destructive steps, rollback-on-trial-failure, migration-aware preflight, state-dir ownership lock. **Improve on it:** add Sigstore/TUF signatures — T3 verifies only a SHA-256 checksum fetched from the *same* release endpoint (no trust root), which is weaker than our planned TUF+Sigstore.
**Maps to:** `S8.7` (secure updates & supply chain — explicitly names "T3 v0.1.10 Windows-service + remote-update pattern with our own rollback gating"), `S5.3` (durable scheduler/backup), `S8.4` (offline/recovery), `S9.9` (distribution).

---

## 4. Anti-patterns / gaps (still missing by design or by neglect)

- **Own agent / tool suite / uniform registry:** none — all agency is in wrapped CLIs. ACP `toolKind` is a thin string (`clientPolicy.ts:82`, `:188`); there is no cross-provider canonical risk taxonomy. *(Our S0.2/S0.4 advantage.)*
- **No browser/OS actuation:** desktop `snapShot/` is capture-only (screenshots + accessibility tree, macOS/Windows/Wayland); no `SendInput`/UIA/keyboard/mouse actuation found. Preview automation (`preview_snapshot`/`preview_click`) acts only on T3's own preview tabs. *(Our S3/S4 advantage.)*
- **No hash chain:** events have monotonic `seq` but are unchained. The audit spine is tamper-*evident* only via DB constraints, not cryptographic. *(Our S0.4/S1.8 hash-chained forensics + SMT monitor advantage.)*
- **Full-access default:** `DEFAULT_RUNTIME_MODE = "full-access"` (`providerPolicy.ts:32`); `auto` mode with no explicit approval policy falls through to `allow` (`clientPolicy.ts:173–182`). The ladder is a UI/session passthrough, not a deny-by-default policy engine; the *admin ceiling* (`DispatchModeLimit`) is the real guardrail. Flip this in our design.
- **Filesystem scope is not a sandbox:** "projects are organizational boundaries, not filesystem sandboxes." The containment utilities protect T3's own read/write endpoints; wrapped CLIs still see the whole machine as the server user. `filesystem:read` is machine-wide.
- **Thin prompt-injection defense:** only webhook payloads get untrusted-data framing (`webhookTemplate.ts`); no Reader/Actor split, no sequence-aware corpus defense, no retrieval quarantine. *(Our S2.13/S3.4 advantage.)*
- **Update integrity is checksum-only:** `SHA256SUMS` fetched from the same host as the archive (`pinnedRuntime.ts:262–284`) — no TUF roles, Sigstore/SLSA provenance, SBOM, or signature verification. Adopt the lifecycle, replace the trust model.
- **Other absences:** no memory system, no general MCP-client management, no TUI, no per-task dollar budgets, no consent ledger / fatigue KPIs / meaningful-oversight test.
- **Anti-pattern to avoid copying:** the 16,160-line `Orchestrator.ts` monolith (grew 27% in 3 weeks). Extract small decision modules; T3's own split (`EventSink`, `EffectOutbox`, `EffectWorker`, `CommandReceiptStore`, `DispatchModeLimit`) shows the right seams — the orchestrator just wasn't refactored.

---

## 5. Sources

All source refs are at tag `v0.1.10` / commit `7d32075e3757ded707914e2450c3e392a9665765`, read from a shallow clone at `C:\Users\ASUS\AppData\Local\Temp\opencode\t3code`:

- `apps/server/src/orchestration-v2/Orchestrator.ts`, `EventSink.ts`, `EffectWorker.ts`, `CommandReceiptStore.ts`, `DispatchModeLimit.ts`, `LiveStreamBudget.ts`, `CheckpointRestoreSafety.ts`, `CheckpointService.ts`, `EffectOutbox.ts`
- `apps/server/src/mcp/McpToolAccess.ts`, `apps/server/src/mcp/toolkits/orchestrator/tools.ts`
- `packages/provider-acp/src/server/clientPolicy.ts`, `packages/provider-acp/src/server/clientTerminals.ts`
- `apps/server/src/workspace/WorkspaceFileSystem.ts`
- `packages/contracts/src/providerPolicy.ts`, `packages/contracts/src/terminal.ts`, `packages/contracts/src/auth.ts`, `packages/contracts/src/managedPolicy.ts`
- `apps/server/src/auth/dpop.ts`, `McpOAuth.ts`, `EnvironmentAuth.ts`, `PayloadSigner.ts`, `SessionStore.ts`
- `apps/server/src/secrets/SecretRequests.ts`
- `apps/server/src/scheduledTasks/webhookVerification.ts`, `webhookTemplate.ts`, `ScheduledTaskService.ts`, `RelayDeliveryProof.ts`
- `oxlint-plugin-t3code/rules/no-raw-mcp-registration.ts`, `no-rpc-permission-bypass.ts`; `vite.config.ts:312`
- `apps/server/src/serviceLauncher.ts`, `apps/server/src/cloud/{selfUpdate,servicePreflight,pinnedRuntime,serviceProtocol,serviceSelectionLock}.ts`
- `packages/client-runtime/src/state/{terminalOutput,threadRetention}.ts`
- `apps/desktop/src/snapShot/` (capture-only confirmation)
- GitHub REST: `api.github.com/repos/pingdotgg/t3code` and `/releases/latest` (freshness, fetched 2026-11-17)
- Local planning: `docs/PLAN.md` §19 & §21, `docs/IMPLEMENTATION-PHASES.md` S-steps.
