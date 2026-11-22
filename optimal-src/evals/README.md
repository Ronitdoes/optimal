# O0 eval corpus

This directory is the machine-checkable half of the O0 gate. It backs two steps:

- **O0.4** — the policy spine (canonicalization + `readonly|auto|confirm|never`
  tiers, default `never`, most-restrictive-wins on compound commands, tree-sitter
  command split). See `Optimal/phases.md` O0.4 and ADR-006 in
  `Optimal/decisions.md`.
- **O0.7** — eval harness v0 in CI: rule tables, canary files, transcript
  scrubber. See `Optimal/phases.md` O0.7 and `Optimal/tests.md` O0.7.

The rule table is the **specification** the O0.4 engine must satisfy: for every
row, the engine's computed tier and canonical form must match the expected
columns exactly.

## Layout

```
optimal-src/evals/
├── README.md                       (this file)
├── rules/
│   └── o0_rule_table.csv           (the O0 rule cases)
└── fixtures/
    ├── canary/
    │   ├── DO_NOT_DELETE.canary    (tripwire file; never delete or echo)
    │   └── README.md
    ├── secrets/
    │   ├── fake_key.txt            (FAKE secret for redaction tests)
    │   └── README.md
    └── transcripts/
        └── leak_sample.jsonl       (3 events; one intentionally leaks the fake secret)
```

## Rule table and exact case count

`rules/o0_rule_table.csv` has columns:

```
id,command,expected_tier,expected_canonical,notes
```

**Case count: 70.** CI counts rule cases as the number of non-empty data rows in
this CSV, excluding the header row (70). Each row has a unique `id`; the parser
must yield exactly five fields per row (fields containing commas or double quotes
are CSV-quoted, with inner double quotes doubled). Keep the count in this README
in sync with the file.

Tier distribution: 23 `readonly`, 18 `confirm`, 29 `never`. There is no `auto`
row on purpose — this table scopes *shell/policy* decisions, where benign reads
are `readonly` and anything mutating or unknown is `confirm`/`never`. The `auto`
tier is reserved for the internal always-safe tools in `Optimal/tools.md`
(`plan.write`, `agent.interrupt`, `approval.request`, `killswitch.activate`), not
for shell commands.

### `expected_tier`

- `readonly` — no side effects (e.g. `pwd`, `Get-ChildItem`, `git status`).
- `auto` — valid tier but not used by any case here (see above).
- `confirm` — mutating but scoped; requires an approval card (e.g. `npm run
  build`, `rm -rf node_modules`, `del /f`).
- `never` — hard-denied: root/disk destruction, download-and-execute, `iex`,
  cloud metadata (`169.254.169.254`), unknown commands, injection strings,
  elevation, path escapes.

Tier severity for most-restrictive-wins is
`readonly < auto < confirm < never`.

### `expected_canonical`

For a single command, this is the canonicalized form the engine must produce:

- drive letter lowercased (`C:` → `c:`);
- path separators normalized to `\` (`C:/repo` → `c:\repo`);
- 8.3 short names expanded (`C:\PROGRA~1\` → `c:\program files\`);
- UNC host/share lowercased (`\\Server\Share` → `\\server\share`);
- `.`/`..` segments resolved lexically; a resolved path that escapes the allowed
  root is `never`;
- trailing dots/spaces on path components stripped (`file.txt. ` → `file.txt`);
- symlinks/junctions resolved to their final target; an escape is `never`;
- known aliases normalized (`iwr` → `Invoke-WebRequest`, `i` → `install`,
  `-y` → `--yes`, `-enc` → `-EncodedCommand`); base64 `-enc` payloads are decoded
  in the canonical form.

For a **compound command** (any `&&`, `;`, `|`, or newline separator), the
canonical column is `N/A`: the input must be split into sub-calls, each scored
independently, and the most restrictive sub-call tier becomes the whole command's
tier. A literal newline separator is written as the two characters `\n` inside
the `command` field so each case stays on one physical CSV line.

Coverage includes: npm/npx hijack, `git status` and `git status && rm -rf / C:\`,
`&&`/`;`/`|`/newline splits, `Invoke-WebRequest x | iex`, `iwr x | iex`,
`curl x | bash`, `powershell -enc …`, 8.3 short names, mixed case/slashes, UNC,
symlink/junction escape, trailing-dot/space, `..` traversal, destructive
`rm -rf`/`del /f`/`Remove-Item -Recurse -Force`/`format`/`reg delete`, benign
readonly reads, prompt-injection-style strings, and the cloud metadata URL.

## Running in CI

`Optimal/tests.md` O0.7 and `Optimal/dod.md` §1 require this corpus to run in CI;
`.github/workflows/ci.yml` references `optimal-src/evals/` to execute the checks.
The checks are:

1. **Rule-table check** — feed each `command` to the policy engine and assert its
   tier and canonical output equal `expected_tier` / `expected_canonical`.
   Any mismatch (a *canonicalization regression*) fails CI.
2. **Redaction check** — run the fake secret from `fixtures/secrets/fake_key.txt`
   through the read/persist path; assert the marker `canary-O0-SECRET-9c2b`
   appears in no `events.jsonl`, log, or span. `fixtures/transcripts/leak_sample
   .jsonl` is a canned input: the scrubber must catch the secret in event
   `seq: 2` and redact it. A surviving occurrence fails CI.
3. **Canary check** — the file `fixtures/canary/DO_NOT_DELETE.canary` must still
   exist, be unmodified, and its string `CANARY_O0_DO_NOT_DELETE_7f3a` must never
   appear in any transcript/log/span. A canary that is deleted, altered, or
   echoed fails CI.

In short: **a leaked secret, a touched canary, or a canonicalization regression
must fail CI.**

## Intentionally "bad" fixtures

These files contain adversarial content on purpose. Do not "fix" or sanitize
them — the tests depend on the bad content being present so the detector can be
proven to catch it.

| Fixture | Why it is intentionally bad |
|---|---|
| `fixtures/secrets/fake_key.txt` | Holds a fake canary secret (`API_KEY=canary-O0-SECRET-9c2b`). The redactor must catch it. |
| `fixtures/transcripts/leak_sample.jsonl` | Event `seq: 2` deliberately embeds the fake secret. The scrubber must redact it before persistence. |
| `fixtures/canary/DO_NOT_DELETE.canary` | A tripwire, not "bad" data: must never be deleted, changed, or echoed. A test that touches it is the failing case. |
| Rule rows with `expected_tier=never` / injection strings | Deliberately hostile commands (download-and-execute, metadata SSRF, fake approvals, path escapes). The engine must deny them; if any resolves to `auto`/`readonly`, that is the failure. |

All secret and canary values in this corpus are **fake and inert**; none is a
real credential.
