# Fake secret fixture — redaction tests

`fake_key.txt` contains exactly:

```
API_KEY=canary-O0-SECRET-9c2b
```

## Rules

- This is a **FAKE canary secret used only for tests**. It is not a real key and
  grants access to nothing.
- It exists so the scrubber/redaction path can be tested with a known marker.
  The value must be redacted before persistence in `events.jsonl`, transcripts,
  logs, and telemetry spans (`phases.md` O0.5, `dod.md` §1).
- Do not replace it with a real credential. Keep the marker stable so CI can
  detect any occurrence deterministically.

See `../transcripts/leak_sample.jsonl` for a transcript that intentionally
embeds this fake secret (the scrubber must catch event `seq: 2`).
