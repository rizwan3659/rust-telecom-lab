# ROADMAP — Rust Telecom Lab (12 months)

> **Status:** v2 — revised 2026-11-17. **Primary focus: high-performance systems and
> performance optimization.** Telecom protocols are the workloads that performance is
> engineered on. v1 assessment findings still apply (see `LEARNING_PROGRESS.md`).

## Weekly time budget (~20 h)

| Activity | v1 | v2 | Why |
|---|---:|---:|---|
| Telecom theory and standards | 5 h | 4 h | Theory only as needed to build realistic workloads |
| Rust implementation | 6 h | 6 h | Unchanged |
| Testing, benchmarking, profiling, optimization | 5 h | **7 h** | Includes ~2 h/week of the **performance lab thread** |
| Documentation and YouTube | 3 h | 2 h | Performance write-ups double as content |
| Community and monetization | 1 h | 1 h | Unchanged |

## The rule for every component

A component is not finished when it works. It is finished when you can show:

1. **Baseline:** throughput (TPS, CPS or pps), p50/p95/p99/p99.9, CPU per core, RSS, with machine specs.
2. **Profile:** a flamegraph or counter data (`perf stat`) explaining where time goes.
3. **Bottleneck statement:** a one-sentence claim backed by the profile ("38% in malloc from per-AVP Vec").
4. **Experiment:** one change, with before/after numbers and the change's effect on correctness tests.
5. **Capacity curve:** latency vs offered load, up to and past the saturation point (from month 3 on).

Correctness tests must pass after every optimization. A faster wrong answer is a regression.

## Phase overview

| Month | Build (telecom workload) | Performance focus | Perf deliverable |
|------:|---|---|---|
| 1 | `diameter-codec`: header, AVPs, grouped AVPs, encoder | Benchmark hygiene, Criterion, flamegraphs, allocation profiling | Single-allocation encoder with before/after; zero-alloc decoder proven |
| 2 | Async Diameter peer: Tokio framing, CER/CEA, DWR/DWA watchdog, session table | Async runtime cost; lock contention: global Mutex vs sharded vs actor, measured | Contention benchmark across 1–16 threads with a written explanation |
| 3 | Open-loop load generator v0 (Diameter) | HdrHistogram, coordinated omission, capacity curves, USE method, false sharing | Capacity curve of your own Diameter server; per-thread stats without false sharing |
| 4 | Zero-copy SIP parser + transaction timers | Data layout, branch prediction, SIMD scanning (`memchr`, AVX2/AVX-512), timer wheels vs heaps | Parser throughput in GB/s and messages/s; timer structure comparison at 1M timers |
| 5 | High-performance UDP signaling path (SIP/Diameter over UDP) + load generator v1 | Syscall batching (`recvmmsg`/`sendmmsg`), GSO/GRO, `SO_REUSEPORT` multi-core scaling, core pinning, IRQ/RSS affinity | Scaling curve 1→8 cores; comparison against SIPp at identical scenarios |
| 6 | 5G-AKA / Milenage (TS 35.206, test vectors TS 35.208) + NAS-5GS decoder (TS 24.501) | Crypto cost per registration, AES-NI, batching crypto work off the async path | Registrations/s per core bound by crypto, measured |
| 7 | NGAP (TS 38.413, ASN.1 APER) + SCTP transport | Decode cost of PER vs TLV; SCTP vs TCP throughput and latency | Decode benchmarks; SCTP vs TCP comparison report |
| 8 | UE/gNB simulator as a **high-scale load generator** (register against Open5GS) | Memory per simulated UE, state-machine layout, 100k+ concurrent UEs, timer scalability | Max simulated UEs within a fixed memory budget, measured |
| 9 | GTP-U forwarder (TS 29.281), stage 1, with a minimal PFCP rule install (TS 29.244) | pps methodology; per-packet cost budget; batching; multi-core RSS; lookup tables for packet detection | pps/core curve; ns-per-packet breakdown |
| 10 | GTP-U forwarder stage 2 | `io_uring`, AF_XDP, lock-free session lookup, prefetching | stage-1 vs stage-2 comparison (see hardware notes) |
| 11 | Observability + continuous benchmarking | eBPF (`bpftrace`), off-CPU analysis, `perf stat` counters, perf-regression CI | CI job that fails on a >X% regression |
| 12 | HA stateful NF: replicated session store, fencing | Cost of replication on p99; failover time; recovery throughput | Failover time and p99 impact, measured under load |

**Moved to year 2:** IMS call lab and the LTE/EPC lab. IMS registration remains as a
load scenario in month 5. DPDK is a year-2 option (needs FFI, `unsafe`, and suitable NICs).

## Performance lab thread (~2 h/week)

Small, self-contained experiments on code you've already written. Each one is a
hypothesis, a measurement, and a short note in `docs/benchmarks/`.

| ID | Experiment | Concept |
|---|---|---|
| PERF-LAB-001 | Make benchmarks reproducible: CPU governor, turbo, core isolation, run-to-run variance | Noise and how to control it |
| PERF-LAB-002 | Cost of heap allocation vs stack vs reused buffer | Allocator cost |
| PERF-LAB-003 | Bounds checks: indexing vs iterators vs fixed-size arrays; read the assembly (`cargo asm`) | What the compiler does with your code |
| PERF-LAB-004 | 100k Tokio tasks vs 100k OS threads: memory and switch latency | Tasks vs threads, measured (assessment Q5) |
| PERF-LAB-005 | False sharing: packed vs cache-padded per-thread counters, 1–16 threads | Cache lines (assessment Q7) |
| PERF-LAB-006 | Sequential vs random memory access; linked list vs `Vec` | Cache misses, prefetching |
| PERF-LAB-007 | `HashMap` hashers (SipHash vs FxHash/ahash) on session-ID keys; security trade-off | Hashing cost vs HashDoS |
| PERF-LAB-008 | Closed-loop vs open-loop load against a server with an injected stall | Coordinated omission (assessment Q8) |
| PERF-LAB-009 | One `recv` per packet vs `recvmmsg` batches | Syscall overhead |
| PERF-LAB-010 | Branch-heavy vs branchless parsing; `perf stat` branch-misses | Branch prediction |

More are added as the build phases raise questions.

## Hardware notes (measured 2026-11-17)

| Resource | Value | Consequence |
|---|---|---|
| CPU | Intel i7-11700, 8 cores / 16 threads, 1 NUMA node | Good for scaling curves to 8 cores. NUMA experiments not possible. |
| SIMD / crypto | AVX2, AVX-512 (F/BW), AES-NI, SHA-NI | SIMD parsing and crypto labs fully possible |
| Caches | L1d 48 KiB/core, L2 512 KiB/core, L3 16 MiB shared | Working-set experiments will show clear steps at these sizes |
| RAM | 15 GiB total, ~4 GiB free at check time | 100k-UE simulation must be memory-budgeted; close other apps for benchmarks |
| CPU governor | `powersave`, turbo on | Benchmarks need PERF-LAB-001 first, or run-to-run noise will hide real effects |
| NIC | Intel `e1000e`, 1 GbE | Real line-rate packet work is limited to 1 Gb/s. AF_XDP runs only in generic (copy) mode: fine for learning the API, **not** for meaningful pps numbers |
| Tools | `perf` installed; `bpftrace`, `heaptrack` not yet | Install as each month needs them |

Single-machine testing: the load generator and the system under test share the CPU.
Pin them to separate cores (`taskset`) and report it. For realistic month-9/10 numbers,
a second machine or a NIC with native XDP support (e.g. Intel X710/E810 class) would be needed.
This is optional; it is noted so results are never over-claimed.

## Weeks 1–4 in detail

| Week | Engineering tasks | Performance lab | Reading (~4 h) |
|---:|---|---|---|
| 1 | **DIA-HDR-001** Diameter header codec | **PERF-LAB-001** reproducible benchmark environment | RFC 6733 §3; Gregg *Systems Performance* ch. 2 (methodologies) |
| 2 | **DIA-AVP-001** zero-copy AVP iterator · **DIA-AVP-002** fuzzing · **PERF-BENCH-001** first Criterion benchmarks | **PERF-LAB-002** allocation cost | RFC 6733 §4; Criterion user guide; *The Rust Performance Book* |
| 3 | **DIA-AVP-003** grouped AVPs · **DIA-ENC-001** single-allocation encoder · **PERF-PROF-001** flamegraph + allocation count, before/after | **PERF-LAB-003** bounds checks and assembly | Drepper *What Every Programmer Should Know About Memory* §3 (caches) |
| 4 | **NET-FRAME-001** Tokio TCP framing · **NET-DIAG-001** accept-queue overflow lab | **PERF-LAB-004** tasks vs threads | RFC 6733 §5; Tokio tutorial; Mara Bos *Rust Atomics and Locks* ch. 1–2 |

## Milestones

- **M1 (end month 3):** Diameter stack + open-loop load generator, with a published capacity curve.
- **M2 (end month 5):** Multi-core UDP signaling path with a scaling curve and a SIPp comparison.
- **M3 (end month 8):** UE/gNB simulator registering against Open5GS, with max-UE and memory-per-UE results.
- **M4 (end month 10):** GTP-U forwarder with stage-1 vs stage-2 pps comparison.
- **M5 (end month 12):** HA NF with failover time and p99 impact measured under load.

## Revision log

- 2026-11-12 — v0 draft created at workspace initialization (pre-assessment).
- 2026-11-16 — v1: rebuilt from assessment results.
- 2026-11-17 — v2: performance becomes the primary focus. Rebalanced weekly hours,
  added the performance lab thread, made each phase's deliverable a performance result,
  added hardware notes, moved IMS call lab to year 2.
