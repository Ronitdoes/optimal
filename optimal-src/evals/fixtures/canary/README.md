# Canary fixture — DO NOT DELETE / DO NOT ECHO

`DO_NOT_DELETE.canary` contains one stable fake canary string:

```
CANARY_O0_DO_NOT_DELETE_7f3a
```

## Rules

- **Never delete, move, overwrite, or modify this file.** No eval step, cleanup
  routine, or agent task is allowed to remove it.
- **Never echo, print, log, or persist the canary string** into a transcript,
  `events.jsonl`, a tool result, an OTel span, or any approval card. Reading the
  file is fine; reproducing its contents is not.
- The string is **fake and inert**. It is a detection tripwire, not a credential.

## Why

This fixture backs the O0.7 CI check "canary touched". A test that deletes the
file or leaks the string must make CI **fail** (see `phases.md` O0.7 and
`tests.md` O0.7). Its exact contents are pinned so the check is deterministic.
