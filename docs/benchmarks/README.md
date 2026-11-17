# Benchmark and performance reports

One file per experiment: `docs/benchmarks/<TASK-ID>.md`. Raw tool output may sit next to it
(`<TASK-ID>/criterion/`, `<TASK-ID>/perf-stat.txt`). Flamegraph SVGs that support a claim
may be committed; bulk `perf.data` files may not.

## Report template

```markdown
# <TASK-ID> — <what was measured>

## Hypothesis
What you expect and why, written *before* measuring.

## Environment
- Commit: <git hash>
- CPU / cores used / pinning: <lscpu model; taskset mask>
- Governor / turbo: <scaling_governor; intel_pstate/no_turbo>
- RAM free at start: <free -h>
- Kernel / rustc: <uname -r; rustc -V>
- Build profile: <release, lto, codegen-units, target-cpu>
- Other load on the machine: <none / browser closed / ...>

## Method
Tool, inputs, warm-up, duration, repetitions, open- or closed-loop and rate (for load tests).

## Results
Table of numbers copied from tool output. Include run-to-run variation.

## Analysis
What the profile shows. One-sentence bottleneck statement.

## Conclusion
Was the hypothesis right? What changes next?
```

## Rules

- Numbers come from real runs only. Never estimate a value and present it as measured.
- Before/after comparisons use the same machine, inputs and settings; change one thing.
- Correctness tests must pass on the "after" version.
- Targets and expectations are labelled as such, separate from results.
