# Optimal — Tool inventory (Windows desktop / GPUI)
### 2026-11-18 · companion to `plan.md`, `phases.md`, `dod.md`, `tests.md`

> **Every tool** uses the same envelope and the same governed path. A tool cannot be registered without a **branded risk declaration** (T3-style private constructor + lint rule); a missing declaration is a compile error.
>
> ```rust
> ToolEnvelope {
>   id, domain, input: Schema, output: Json|Text|Artifact,
>   risk: RiskDeclaration,        // branded: reads / writes / acts_as_caller / writes_threads / ...
>   permissions: ["fs:write", "net:none", "elevate:false"],
>   approval: "auto|confirm|never", default_tier: "readonly|auto|confirm|never",
>   side_effect_level: 0..3, idempotent: bool, dry_run: bool,
>   timeout_ms, retries {max, backoff_ms}, probe {cmd, ttl_ms}, correlation_id
> }
> Result { status: ok|error|denied|timeout, stdout, stderr, artifacts[], duration_ms, correlation_id }
> ```
> Default for anything unknown or undeclared: **`never`**. Unattended mode: `confirm` ⇒ skip + notify.

---

## 1. Tools by phase (build order)

### O0 — Skeleton: core loop, registry, safety spine
| Tool ID | Purpose | Default tier |
|---|---|---|
| `fs.read` | Read a file with redaction at the sink | readonly |
| `fs.list` | List a directory | readonly |
| `fs.search` | Filename/content search (Everything + ripgrep) | readonly |
| `env.probe` | Detect installed toolchains; probe all tool availability | readonly |
| `shell.exec` | PowerShell 7 via `-EncodedCommand`, tree-sitter split, most-restrictive rule wins | confirm |
| `plan.write` / `todo.update` | Maintain `plan.json` + `todos.md` (plan-as-file) | auto |
| `agent.interrupt` / `agent.kill` | Soft steering / hard kill | auto |
| `tools.search` | Meta-tool: discover capabilities on demand | readonly |
| `policy.check` | Rule playground: paste a command, see matched rule + tier + canonicalization | readonly |
| `approval.request` | Render a full-context confirm card (diff, scope, rule, revert target) | auto |
| `audit.verify` / `audit.export` | Verify hash chain / export a task's receipts | readonly |
| `killswitch.activate` | Global stop (hotkey + tray + CLI) | auto |

### O1 — Code agent, git & boundary
| Tool ID | Purpose | Default tier |
|---|---|---|
| `git.status` / `git.diff` / `git.log` | Porcelain-v2 machine-parsed git | readonly |
| `git.add` / `git.commit` | Staged commits with checkpoint-before-mutate | confirm |
| `git.snapshot` | Capture content-addressed tree in the **separate** snapshot repo | auto |
| `git.rewind` | Three-axis restore: `code|conversation|both` | confirm |
| `git.worktree.create` / `git.worktree.remove` | Worktree-per-task with ownership + fencing | confirm |
| `undo.apply` | Apply the global undo ledger entry | confirm |
| `egress.check` | "What left the machine" receipt; destination/inference policy check | readonly |
| `consent.grant` / `consent.revoke` / `consent.list` | Consent Ledger: scope, TTL, revocation, signed receipts | confirm / auto / readonly |
| `secret.get` | `{{vault:ref}}` substitution inside executor only; never into context | auto |
| `budget.status` | Per-task token/$ / tool-count / bytes meters | readonly |
| `finish_gate.verify` | Clean-context re-verification of claimed artifacts (≤3 repair rounds) | auto |

### O2 — Durability, context, telemetry
| Tool ID | Purpose | Default tier |
|---|---|---|
| `session.resume` / `session.fork` | Reattach / branch a session from its event log | auto |
| `context.compact` | Measured-token compaction, shrink ladder, `keepID` | auto |
| `context.checkpoint` | Checkpoint before compaction/mutation | auto |
| `memory.read` / `memory.search` | File-first memory + FTS; stale facts re-validated | readonly |
| `memory.write` | **Staged** memory write with diff + approve + TTL | confirm |
| `event.replay` | Replay a session from events (CI + "why did you do that") | readonly |
| `trace.export` / `cost.report` | OTel spans (content OFF) / per-task cost | readonly |

### O3 — GPUI UX-facing (tools the UI calls, not the model)
| Tool ID | Purpose |
|---|---|
| `ui.task_tree` / `ui.tool_badge` / `ui.cost_footer` | Streaming projection into GPUI views |
| `ui.permission_card` | Full-context card with ordered decisions + counterfactual "why" |
| `ui.ambient_state` | 4-state channel: idle / progressing / needs-you / stuck |
| `ui.takeover` | Graduated handover + pre-staged context packet |
| `ui.capture_region` | Screen-region capture from GPUI/Win32 |
| `ui.notify` | Tray/notification-centre push with deep link |

### O4 — Tools, MCP & extensibility
| Tool ID | Purpose | Default tier |
|---|---|---|
| `fs.write` / `fs.edit` | Whole-file or hashline-anchored edits with dry-run diff | confirm |
| `fs.delete` | Recycle-Bin only; never permanent | confirm |
| `fs.move` / `fs.copy` / `fs.extract` | OneDrive placeholder recall; zip-slip rejection | confirm |
| `code.search` | `rg --json` structured content search | readonly |
| `ast.edit` | tree-sitter structural edit, staged preview, atomic apply | confirm |
| `lsp.diagnostics` / `lsp.rename` / `lsp.references` | Structured diagnostics; semantic rename preferred over text edit | readonly / confirm |
| `code.execute` | **Code Mode**: confined `rhai`, host budgets, nested permission re-check, protected params | confirm |
| `mcp.list` / `mcp.probe` / `mcp.call` | Namespaced `mcp/server.tool`; 2026-07-28 dual transport; pinning + description scan | readonly / confirm |
| `plugin.install` / `plugin.verify` | Vetted install, hash-trusted hooks, scoped revision + auto-disable | confirm |
| `skill.install` / `skill.list` / `skill.diff` / `skill.approve` / `skill.lock` | Staged skill writes, hash + source + scanner version, per-skill lock | confirm |
| `package.install` | OSV MAL-* gate, **fail-closed** on unknown ecosystem/token | confirm |

### O5 — Windows GUI automation
| Tool ID | Purpose | Default tier |
|---|---|---|
| `uia.snapshot` | UIA tree digest (ControlType, Name, AutomationId, rect), cached, invalidated on structure change | readonly |
| `uia.find` / `uia.invoke` / `uia.set_value` | Find by a11y ref; act through UIA patterns | readonly / confirm |
| `ocr.recognize` | Windows.Media.Ocr first, vision fallback for irreducibly visual steps | readonly |
| `screen.capture` | Full screen/window capture for post-action verification | readonly |
| `input.click` / `input.type` / `input.key` | SendInput last resort; per-monitor DPI-aware | confirm |
| `window.list` / `window.move` | Window management (snap, virtual desktops) | auto |
| `app.com` | Office COM scripting where available | confirm |
| `desktop.lock_check` | Presence awareness; scheduled jobs skip when locked | readonly |
| `desktop.acquire` / `desktop.release` | Single desktop lock (one user's desktop is serial) | auto |

### O6 — Guardian, scheduling, background
| Tool ID | Purpose | Default tier |
|---|---|---|
| `schedule.cron` / `schedule.event` / `schedule.webhook` | Durable triggers, one-use HMAC webhook ingress, dedupe/rate-limit | confirm |
| `job.status` / `job.cancel` / `job.revive` | Recoverable background jobs with notify-on-finish | readonly / auto |
| `guardian.review` | Separate reviewer, advisory; circuit breaker; policy-revision-scoped scores | auto |
| `two_key.request` / `two_key.sign` | N-of-M attestors for irreversible ops | confirm |
| `task.spawn` / `task.status` / `task.message` / `task.kill` | Schema-validated subagents; worktree isolation; async mailbox, not peer chat | confirm |
| `notify.send` | Outbound draft notification (never an external send) | auto |

### O7 — Hardening & release
| Tool ID | Purpose | Default tier |
|---|---|
| `update.check` / `update.verify` / `update.apply` | TUF + Sigstore + SLSA, verify-before-apply, blue-green with rollback | confirm |
| `install.verify` | "Verify current install"; signature + SBOM check | readonly |
| `offline.verify` | Startup self-check proving zero outbound spans in offline profile | readonly |
| `resource.status` | Transcript/checkpoint/RSS caps; archival dashboard | readonly |
| `redaction.verify` | Prove no secret/canary in last N events | readonly |

### O8 — Productization
| Tool ID | Purpose | Default tier |
|---|---|---|
| `hermes.migrate --dry-run` | Import Hermes/OpenClaw skills, memories, allowlists into the vetted pipeline (no key import) | confirm |
| `explain.why` | Time-travel "why did you do that?" bundle from receipts | readonly |

---

## 2. The nine tools that are the moat

Most of the inventory is table stakes copied from the field. These nine are the differentiators — they are what nobody else ships well:

1. **`git.snapshot` + `git.rewind`** — three-axis rewind in a separate repo; "can't roll back conversation → reject before touching files."
2. **`policy.check`** — paste a command, see the exact canonicalized rule, tier, and compound-command split. Transparent governance as a product surface.
3. **`audit.verify` / `audit.export` / `explain.why`** — hash-chained receipts and a court-grade replay of *why* an action happened.
4. **`egress.check`** — a receipt for every byte that left the machine, with L7 + inference-routing policy and a client-side kill switch.
5. **`consent.grant` / `consent.revoke`** — consent as a scoped, TTL-bound, revocable, signed object — not a confirm button.
6. **`code.execute`** — Code Mode with *host-owned* budgets and per-inner-call permission re-check, so batching never bypasses policy.
7. **`task.spawn`** — subagents with schema-validated yields, worktree isolation, and attributable elevation (async mailbox, never free-form chat).
8. **`guardian.review` + `two_key.sign`** — separate reviewer hardened by Z3-validated policy; the model can never edit its own gate.
9. **`offline.verify`** — trace-proven zero outbound calls, which Hermes, Dots, and Grok all lack.

---

## 3. Tools explicitly NOT built in this track

| Deferred | Reason |
|---|---|
| Browser tools (`browser.navigate`, `browser.click`, `browser.fill`, extension bridge) | Needs its own injection corpus; separate capability |
| Email/calendar (`mail.draft`, `mail.send`, `cal.find_slot`) | Comms phase, draft-only + confirm |
| Cloud CLI adapters (`aws.*`, `gh.*`, `az.*`, `kubectl.*`) | After the desktop loop is trustworthy |
| Payments (`pay.token`, `pay.confirm`) | Later, with two-key gate; no card data in context |
| Multi-tenant / household tools (`tenant.switch`, `delegate.mint`) | Deferred product |
| i18n/locale/time tools (`locale.set`, `time.calendar`) | Global track later |
| Social OS tools (`standup.emit`, `claim.attest`, `ledger.commitment`) | Deferred on the receipts substrate |

---

## 4. Invariants every tool obeys

- Deny-by-default; unknown tool = `never`, unknown action = `never`.
- Authorization rechecked **after awaits, under the commit lock** (TOCTOU).
- No effect without a receipt; every mutating tool writes an event + projection + outbox row in one transaction.
- Dry-run preview for every destructive effect; Recycle Bin for deletes.
- Probe before availability; flapping probes use last-good grace, never silently strip a toolset.
- No secret, card number, or untrusted-content text reaches a mutating tool without a receipt.
- Unattended ⇒ `confirm` resolves to skip + notify; never auto-approve.
- Every tool declaration is compile-time branded; lint bans raw registration.

*Source of each mechanism: `../docs/BEST-OF-ADOPTION.md`, `AGENT-RESEARCH-01..09*.md`. Step mapping: `phases.md`.*
