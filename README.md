# rust-telecom-lab

A public engineering log: learning to build 4G/5G/IMS protocol stacks, UE/RAN simulators
and telecom test tools in Rust, one small, tested and benchmarked task at a time.

The author has about 13 years of experience in C/C++, SIP, Diameter, IMS and SS7. This repo
records the move to Rust and to 5G Core, including the gaps found along the way.

## Where to look

| File | Contents |
|---|---|
| [`ROADMAP.md`](ROADMAP.md) | 12-month plan and how it was adapted after the initial assessment |
| [`LEARNING_PROGRESS.md`](LEARNING_PROGRESS.md) | Current task, completed tasks with evidence, benchmark results |
| [`docs/tasks/`](docs/tasks/) | One spec per task: theory, standards, acceptance criteria, tests |
| [`docs/benchmarks/`](docs/benchmarks/) | Raw benchmark results with machine specs |
| `crates/` | Libraries (protocol codecs, stacks) |
| `apps/` | Binaries (simulators, tools, load generators) |

## Ground rules

- Every task has written acceptance criteria and is marked done only with evidence (test output, benchmark data).
- Benchmark numbers come from real runs, with the hardware and toolchain recorded.
- Simplified demonstrations are labelled as such and never presented as standards-complete implementations.

## Status

Month 1: Rust foundations via the Diameter wire format (RFC 6733).
Current task: [DIA-HDR-001: Diameter header codec](docs/tasks/DIA-HDR-001.md).
