# ROADMAP — Rust Telecom Lab (12 months)

> **Status:** v1 — revised 2026-11-16 after the 10-question knowledge assessment
> (results in `LEARNING_PROGRESS.md`). Assumes ~20 h/week
> (5 theory · 6 Rust · 5 test/bench · 3 docs/video · 1 community).

## What the assessment changed

| Finding | Consequence for the plan |
|---|---|
| Strong application-level IMS/Diameter knowledge (Cx flows, watchdog, failover, split brain) | Diameter stays the **first** protocol: the telecom side is familiar, so all new effort goes into Rust. |
| Applied Rust ownership/lifetimes and the async/Tokio model are weak | Month 1 is **synchronous**, pure parsing (borrowing, slices, error types). Async starts only in month 2. Pace is slower than v0. |
| No performance-engineering background | Benchmarking starts in **week 2** with Criterion on code you wrote, so every perf concept is learned on your own measurements. |
| Load-testing models (open/closed loop, coordinated omission) new | Load generator built only after the Diameter stack exists (month 3), with these concepts as explicit acceptance criteria. |
| 5G procedures, AKA mechanics, PFCP mostly unknown | A **parallel theory thread** (5 h/week) covers 5G registration and AKA from month 1, so 5G implementation in month 6 starts with the theory already done. |
| Linux network diagnosis weak | Hands-on diagnosis labs (`ss`, `nstat`, accept-queue overflow) attached to the first networking tasks. |

## Phase overview

| Month | Phase | Tracks | Main deliverable | Interop / reference target |
|------:|-------|--------|------------------|----------------------------|
| 1 | Rust foundations via the Diameter wire format | F, A, G | `diameter-codec` crate: header + AVP decode/encode (incl. grouped), error model, fuzzing, first Criterion benchmarks and allocation profiling | Wireshark sample captures |
| 2 | Async Rust + Diameter peer | F, A, D | Tokio TCP framing, CER/CEA, DWR/DWA watchdog state machine (RFC 3539), shared session table done right; Linux diagnosis lab | freeDiameter or Open5GS HSS |
| 3 | Failover + load generator v0 | D, G | Peer failover with T flag / End-to-End ID; open-loop Diameter load generator with HdrHistogram and coordinated-omission correction; first profile → optimize cycle | Against own peer and freeDiameter |
| 4 | SIP parsing and transactions (RFC 3261) | D, F | Zero-copy SIP parser, UDP/TCP transport, client transaction timers (A/B/E/F) | Kamailio / Asterisk |
| 5 | IMS registration + load generator v1 | D, G | REGISTER with IMS-AKA against a Cx-backed HSS stub; SIP load scenarios compared against SIPp | SIPp, Kamailio IMS |
| 6 | 5G security + NAS-5GS | C, F | Milenage + 5G-AKA key derivation verified with official test vectors (TS 35.208, TS 33.501 Annex A); NAS-5GS decoder (TS 24.501) | Open5GS / UERANSIM pcaps |
| 7 | NGAP + SCTP | C, A | NGAP (TS 38.413, ASN.1 APER) decoding, SCTP transport, pcap → decoded dump tool | UERANSIM captures |
| 8 | UE / gNB control-plane simulator | C, E | Simulated gNB + UE: NG Setup, Registration (5G-AKA), PDU Session Establishment | Open5GS AMF/SMF |
| 9 | PFCP / UPF lab | C, F, G | PFCP (TS 29.244) codec, association + heartbeat with Recovery Time Stamp, session establishment; GTP-U (TS 29.281) forwarder with pps benchmarks | Open5GS SMF |
| 10 | IMS call lab | D | Toy P-CSCF/S-CSCF chain, INVITE flow, Cx to HSS stub, failure scenarios | Kamailio IMS |
| 11 | Observability + profiling tools | G | Prometheus metrics, tracing, flamegraph/eBPF workflow, perf-regression CI | — |
| 12 | Distributed / HA components | H | Replicated session store, quorum + fencing tokens, failover tests with `kill -9` and `tc netem` | Chaos tests |

**Moved out of year 1:** the LTE/EPC lab (S1AP, NAS-EPS, S6a, GTPv2-C). 5G covers the same
concepts with a cleaner design, and the Diameter stack from months 1–3 makes S6a quick later.
It becomes the first phase of year 2. You can override this.

Track E (RAN below RRC: PDCP/RLC/MAC/PHY) is studied conceptually alongside months 7–8;
all RAN work in year 1 is **control-plane simulation, not a real radio implementation**.

## Weeks 1–4 in detail

Each task is 1–4 h. Theory reading runs alongside (~5 h/week).

| Week | Engineering tasks | Theory reading |
|---:|---|---|
| 1 | **DIA-HDR-001** Diameter header codec (workspace setup included) | RFC 6733 §1–3; Rust Book ch. 4 and 10 (ownership, lifetimes) re-read with this code in mind |
| 2 | **DIA-AVP-001** Zero-copy AVP iterator (borrowed slices) · **DIA-AVP-002** fuzzing + negative tests · **PERF-BENCH-001** first Criterion benchmark + machine-spec recording | RFC 6733 §4–4.3; TS 33.501 §6.1.3.2 (5G AKA); Criterion user guide |
| 3 | **DIA-AVP-003** grouped AVPs + typed accessors · **DIA-ENC-001** single-allocation encoder with length back-patching · **PERF-PROF-001** flamegraph + allocation counts, naive vs single-alloc before/after | RFC 6733 §4.4; TS 23.502 §4.2.2.2 (5G registration) |
| 4 | **NET-FRAME-001** Tokio TCP framing (partial reads, max size, `Bytes`) · **NET-DIAG-001** reproduce and diagnose accept-queue overflow with `ss` / `nstat` · first video outline | RFC 6733 §5 (peer connections), RFC 3539 (watchdog); Tokio tutorial |

## Per-phase exit criteria (applies to every phase)

1. Correctness: unit, negative, boundary and integration tests pass.
2. Interop: at least one capture of an exchange with a reference implementation, or a documented reason why not yet.
3. Performance: baseline benchmark, p50/p95/p99, CPU/RSS, recorded with machine specs.
4. Optimization: one profiled bottleneck with before/after numbers.
5. Content: README, short article, video outline.

## Milestones

- **M1 (end month 3):** Public `diameter-codec` + peer crate with benchmarks and an open-loop load generator.
- **M2 (end month 5):** IMS registration demo plus a SIP/Diameter load tool benchmarked against SIPp.
- **M3 (end month 8):** UE/gNB simulator registering against Open5GS.
- **M4 (end month 10):** IMS call-flow lab.
- **M5 (end month 12):** HA stateful NF demo with failover measurements.

## Revision log

- 2026-11-12 — v0 draft created at workspace initialization (pre-assessment).
- 2026-11-16 — v1: rebuilt from assessment results. Month 1 synchronous Rust foundations;
  async moved to month 2; perf methodology from week 2; LTE/EPC moved to year 2;
  parallel 5G theory thread added.
