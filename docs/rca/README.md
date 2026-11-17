# Root-cause analysis library

One report per debug, traffic or real-system drill: `docs/rca/<DRILL-ID>.md`.
Keep each to about one page. The aim is a searchable record of how real problems were found.

## Template

```markdown
# <DRILL-ID> — <one-line symptom>

**Date:** <YYYY-MM-DD> · **Time box:** <h> · **Time taken:** <h> · **Solved unaided / with hints / with AI:** <which>

## Symptom
What was observed, with numbers (e.g. "p99 rose from 2 ms to 480 ms at 3k TPS; CPU 35%").

## Evidence collected
Commands run and the key lines of output (perf, ss, nstat, tshark, logs). Attach files if large.

## Hypotheses
| # | Hypothesis | Test | Result |
|---|---|---|---|
| 1 | ... | ... | confirmed / rejected |

Mark which hypotheses came from you and which from the AI.

## Root cause
One or two sentences, backed by the evidence above.

## Fix and verification
The change, and the measurement proving the symptom is gone. Tests still pass.

## Lessons
What to check first next time. Any tool or technique learned.
```
