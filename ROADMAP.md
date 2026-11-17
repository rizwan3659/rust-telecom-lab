# ROADMAP — Rust Telecom Lab (12 months)

> **Status:** v3 — revised 2026-11-17.
> **Goal:** become the engineer who, working with Claude Code, can build high-performance
> telecom systems, find bottlenecks, debug complex traffic, and verify AI-written Rust.
> Telecom protocols are the workloads; performance and diagnosis are the skills.

## Working model: AI-paired engineering

Claude Code writes most of the implementation code. You own everything that decides whether
the result is correct and fast. Typing code is cheap; judgment and evidence are not.

| Step | Who | What |
|---|---|---|
| 1. Specify | **You** | Write the task's behaviour, acceptance criteria and test cases from the RFC / 3GPP spec |
| 2. Predict | **You** | Write down the expected bottleneck and rough performance *before* any code exists |
| 3. Implement | Claude | Implementation, from your spec (except foundation and kata tasks, which you write) |
| 4. Verify | **You** | Read every line on the critical path; run tests, clippy, fuzzing; check spec conformance |
| 5. Measure | **You** | Benchmark, profile, load-test; collect the evidence |
| 6. Diagnose | You + Claude | Claude proposes hypotheses from your evidence; you confirm or reject each with a measurement |
| 7. Explain | **You** | Explain why the code is correct and why it performs as it does; compare with step 2 |

**Rule:** no claim is accepted without evidence, whether it comes from you, from Claude, or
from a blog post. That includes Claude's claims about 3GPP behaviour.

### Task types

| Type | ID prefix | What you train |
|---|---|---|
| Foundation (hand-written) | DIA-HDR, DIA-AVP-001 | Ownership/borrowing intuition needed to review Rust at all |
| Kata (hand-written, ~1–2 h, every ~3 weeks) | KATA-xxx | Keeping hands-on fluency in async, atomics, unsafe review |
| AI-paired build | DIA-, NET-, SIP-, NGAP-, UPF- … | Specification, verification, measurement |
| Performance lab | PERF-LAB-xxx | Measured experiments on hardware behaviour |
| Review drill | REVIEW-DRILL-xxx | Claude writes code with planted correctness, performance and panic bugs; you find them |
| Debug drill | DEBUG-DRILL-xxx | A fault is injected into a running system; you find the root cause within a time box |
| Traffic drill | TRAFFIC-xxx | pcap analysis with Wireshark/tshark: latency, retransmissions, failing peer or procedure |
| Real-system drill | REAL-xxx | Load and profile Open5GS / UERANSIM / Kamailio; find their bottlenecks |

Every debug, traffic and real-system drill produces a root-cause report in `docs/rca/`.

## Weekly time budget (~20 h)

| Activity | Hours |
|---|---:|
| Telecom theory and standards (still essential: it's how you catch AI being wrong) | 4 |
| Specifying, reviewing, and hand-writing foundation/kata code | 6 |
| Benchmarking, profiling, perf labs, debug/traffic drills | 7 |
| Documentation, RCA write-ups, YouTube | 2 |
| Community, upstream contributions, monetization | 1 |

## Definition of done for every component

1. **Correctness:** tests from your spec pass, including negative, boundary and malformed-input (fuzz) tests.
2. **Review:** you can explain every line on the critical path.
3. **Baseline:** throughput (TPS, CPS or pps), p50/p95/p99/p99.9, CPU per core, RSS, with machine specs.
4. **Profile:** flamegraph or `perf stat` evidence of where time goes.
5. **Bottleneck statement:** one sentence, backed by the profile.
6. **Experiment:** one change with before/after numbers; correctness tests still pass.
7. **Capacity curve:** latency vs offered load, past saturation (from month 3 on).
8. **Prediction check:** how your step-2 prediction compared with reality.

## Phase overview

| Month | Build | Performance focus | Drills and real systems | Perf deliverable |
|------:|---|---|---|---|
| 1 | `diameter-codec` (header + AVP iterator hand-written; encoder AI-paired) | Benchmark hygiene, Criterion, flamegraphs, allocation profiling | First review drill; Diameter pcaps with tshark | Zero-alloc decoder proven; single-alloc encoder before/after |
| 2 | Async Diameter peer: framing, CER/CEA, DWR/DWA watchdog, session table | Async runtime cost; lock contention (global vs sharded vs actor) | Debug drills: blocking call in async, lock contention, slow peer | Contention benchmark across 1–16 threads |
| 3 | Open-loop load generator v0 (Diameter) | HdrHistogram, coordinated omission, capacity curves, USE method | Load and profile freeDiameter; **re-assessment #1** | Capacity curve of your server and of freeDiameter |
| 4 | Zero-copy SIP parser + transaction timers | Data layout, SIMD scanning, timer wheels vs heaps | SIP pcap drills: retransmissions, Timer A/B/F behaviour | Parser GB/s; timer structures at 1M timers |
| 5 | Multi-core UDP signaling path + load generator v1 (IMS REGISTER scenario) | `recvmmsg`/`sendmmsg`, GSO/GRO, `SO_REUSEPORT`, pinning, IRQ/RSS | **REAL:** load and profile Kamailio; compare with SIPp | Scaling curve 1→8 cores |
| 6 | 5G-AKA / Milenage + NAS-5GS decoder | Crypto cost per registration, AES-NI | Verify AI-written crypto against TS 35.208 test vectors; Open5GS + UERANSIM lab setup; **re-assessment #2** | Registrations/s per core, crypto-bound |
| 7 | NGAP via a generated ASN.1 APER codec (evaluate existing Rust ASN.1 crates) + SCTP | PER vs TLV decode cost; SCTP vs TCP | NGAP pcap drills: failed registrations, SCTP issues | Decode benchmarks; SCTP vs TCP report |
| 8 | UE/gNB simulator as a high-scale load generator | Memory per UE, state-machine layout, 100k+ UEs | **REAL:** find the Open5GS AMF bottleneck under your load (profiling C code with perf) | Max UEs within a memory budget; Open5GS capacity curve |
| 9 | GTP-U forwarder stage 1 + minimal PFCP rule install | pps methodology, per-packet budget, batching, multi-core | Debug drills: packet loss, PFCP heartbeat/restart scenarios; **re-assessment #3** | pps/core curve, ns-per-packet breakdown |
| 10 | GTP-U forwarder stage 2 | `io_uring`, AF_XDP, lock-free lookup, prefetching | Compare against Open5GS UPF | Stage-1 vs stage-2 comparison |
| 11 | Observability + continuous benchmarking | eBPF (`bpftrace`), off-CPU analysis, `perf stat` counters | Debug drills using only production-style telemetry | CI job failing on perf regression |
| 12 | HA stateful NF: replicated session store, fencing | Replication cost on p99; failover time | Chaos drills: `kill -9`, `tc netem` partitions | Failover time and p99 impact under load |

**Moved to year 2:** IMS call lab, LTE/EPC lab, DPDK.

## Milestones — measured by what you can diagnose

| Milestone | Build result | Diagnosis capability (time-boxed drill) |
|---|---|---|
| **M1** (end month 3) | Diameter stack + open-loop load generator, capacity curve published | Find an injected fault in your Diameter server with evidence within 2 h |
| **M2** (end month 5) | Multi-core UDP signaling path, SIPp comparison | Explain Kamailio's bottleneck under your load with a flamegraph |
| **M3** (end month 8) | UE/gNB simulator registering against Open5GS at scale | Diagnose a failing or slow 5G registration from pcaps + logs within 2 h |
| **M4** (end month 10) | GTP-U forwarder, stage-1 vs stage-2 | Locate a pps drop between NIC, kernel and application |
| **M5** (end month 12) | HA NF with failover measurements | Run a chaos drill and produce a full RCA with timeline |

## Performance lab thread (~2 h/week)

| ID | Experiment | Concept |
|---|---|---|
| PERF-LAB-001 | Reproducible benchmarks: governor, turbo, core isolation, run-to-run variance | Noise control |
| PERF-LAB-002 | Heap vs stack vs reused buffer | Allocator cost |
| PERF-LAB-003 | Indexing vs iterators vs fixed arrays; read the assembly | Bounds checks, codegen |
| PERF-LAB-004 | 100k Tokio tasks vs 100k OS threads | Tasks vs threads (assessment Q5) |
| PERF-LAB-005 | Packed vs cache-padded per-thread counters, 1–16 threads | False sharing (Q7) |
| PERF-LAB-006 | Sequential vs random access; linked list vs `Vec` | Cache misses, prefetching |
| PERF-LAB-007 | SipHash vs FxHash/ahash on session-ID keys | Hashing cost vs HashDoS |
| PERF-LAB-008 | Closed- vs open-loop load against an injected stall | Coordinated omission (Q8) |
| PERF-LAB-009 | One `recv` per packet vs `recvmmsg` | Syscall overhead |
| PERF-LAB-010 | Branchy vs branchless parsing; `perf stat` branch-misses | Branch prediction |

## AI workflow skills (practised in every task)

- Turning evidence into text the AI can use: `perf report --stdio`, `perf stat`, `tshark -T fields`, `ss -tni`, test output.
- Asking for **hypotheses and a way to test each one**, not for fixes.
- Giving context through `CLAUDE.md`, task specs and acceptance tests, so generated code is held to your spec.
- Using a second AI pass (review agents, `/code-review`) as one input, never as the verdict.
- Spotting AI failure patterns: invented spec references, untested edge cases, unmeasured performance claims.

## Hardware notes (measured 2026-11-17)

| Resource | Value | Consequence |
|---|---|---|
| CPU | Intel i7-11700, 8 cores / 16 threads, 1 NUMA node | Scaling curves to 8 cores; no NUMA experiments |
| SIMD / crypto | AVX2, AVX-512 (F/BW), AES-NI, SHA-NI | SIMD and crypto labs fully possible |
| Caches | L1d 48 KiB/core, L2 512 KiB/core, L3 16 MiB shared | Working-set steps visible at these sizes |
| RAM | 15 GiB, ~4 GiB free at check time | Memory-budget the 100k-UE simulator; close other apps for benchmarks |
| CPU governor | `powersave`, turbo on | PERF-LAB-001 first, or noise hides real effects |
| NIC | Intel `e1000e`, 1 GbE | AF_XDP generic mode only: OK for learning, **not** for meaningful pps numbers |
| Tools | `perf` installed; `bpftrace`, `heaptrack`, `tshark` to install when needed | — |

Single machine: pin load generator and system under test to separate cores and report it.
A second machine or a NIC with native XDP (e.g. Intel X710/E810 class) is optional, for
publishable month-9/10 results.

## Weeks 1–4 in detail

| Week | Build | Perf lab / drills | Reading (~4 h) |
|---:|---|---|---|
| 1 | **DIA-HDR-001** header codec — *hand-written* | **PERF-LAB-001** reproducible benchmarks · **TRAFFIC-001** tshark basics on a Diameter sample capture | RFC 6733 §3; Gregg *Systems Performance* ch. 2 |
| 2 | **DIA-AVP-001** zero-copy AVP iterator — *hand-written* · **PERF-BENCH-001** first Criterion benchmarks (you design what to measure; AI-paired harness) | **REVIEW-DRILL-001** AI-written AVP encoder with planted bugs · **PERF-LAB-002** | RFC 6733 §4; *The Rust Performance Book* |
| 3 | **DIA-ENC-001** encoder — *AI-paired*: your spec, tests and prediction; Claude implements naive then single-alloc · **DIA-AVP-002** fuzzing | **PERF-PROF-001** flamegraph + allocation counts, before/after · **PERF-LAB-003** | RFC 6733 §4.4; Drepper *What Every Programmer Should Know About Memory* §3 |
| 4 | **NET-FRAME-001** Tokio TCP framing — *AI-paired* · **NET-DIAG-001** accept-queue overflow lab | **DEBUG-DRILL-001** p99 spike in an async server (injected fault) · **PERF-LAB-004** | RFC 6733 §5; Tokio tutorial; Mara Bos *Rust Atomics and Locks* ch. 1–2 |

## Revision log

- 2026-11-12 — v0 draft created at workspace initialization (pre-assessment).
- 2026-11-16 — v1: rebuilt from assessment results.
- 2026-11-17 — v2: performance becomes the primary focus; perf lab thread; hardware notes.
- 2026-11-17 — v3: AI-paired engineering model. Hand-written foundation limited to
  DIA-HDR-001 and DIA-AVP-001 plus periodic katas; added review, debug, traffic and
  real-system drills; RCA library; quarterly re-assessment; malformed-input testing;
  upstream contributions; milestones measured by diagnosis capability.
