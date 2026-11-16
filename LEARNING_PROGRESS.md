# LEARNING PROGRESS

_Last updated: 2026-11-16_

## Current position

- **Phase:** Month 1 — Rust foundations via the Diameter wire format (ROADMAP v1)
- **Track:** F (Advanced Rust) + A (binary protocols)
- **Current task:** DIA-HDR-001 — Diameter message header codec (`docs/tasks/DIA-HDR-001.md`), assigned 2026-11-16
- **Assessment:** ASSESS-000 completed 2026-11-16 (10 / 10 answered)

## Assessment log

| # | Area | Question (short) | Assessment of answer |
|--:|------|------------------|----------------------|
| 1 | Telecom architecture | Why CUPS; PFCP rules (PDR/FAR/QER/URR); N4 failure recovery | **Partial.** Got 2 of 3+ reasons for the split (independent scaling, edge placement for latency). PFCP rule model: not known. N4 failure / restart recovery: not known. |
| 2 | LTE / 5G signaling | 5G registration steps; RAND/AUTN and mutual auth; relation to IMS-AKA | **Weak.** Knows only the top level (register at AMF, then PDU session via SMF); no RRC/NGAP/AUSF/UDM steps or interfaces. AKA: knows the vector contains RAND, AUTN, CK, IK (missed XRES). Misconception: thought the UE extracts the secret key from the challenge. AUTN / MAC / SQN and network authentication: not known. No comparison of access AKA with IMS-AKA given. |
| 3 | IMS / Diameter | Cx flow at registration; stalled-peer detection, failover, duplicate detection; production debugging story | **Good on basics, gaps in protocol mechanics.** Correct: I-CSCF role and UAR, MAR for vectors, SAR for profile, watchdog for detection, failover to peer B. Error: placed MAR on the 2nd REGISTER (it is on the 1st). Not known: T flag / End-to-End Identifier duplicate detection, Tw timing. Production story: not given. |
| 4 | Rust ownership | Zero-copy `Avp<'a>` returned from a function owning the buffer; redesigns; sending to another task | **Weak.** Misdiagnosed the error as a mutable/immutable borrow conflict (actual: returning a borrow of a local buffer, i.e. use-after-free in C). Redesign options (owned data, caller-owned buffer, `Bytes`): not known. `Send` / `'static` constraints for spawn and channels: not known. |
| 5 | Rust concurrency / async | Lock held across `.await`; restructuring; tasks vs threads, blocking in async | **Weak / partial.** Correct intuition that other threads block on the shared lock. Missed: global serialization (~200 req/s cap), worker-thread starvation, `MutexGuard` is `!Send` so `tokio::spawn` rejects it, tokio Mutex suspends the task (not the thread) and still serializes. Restructuring (copy out, release, re-lock, check-and-set / actor per session): not known. Task vs thread, cost of blocking in async: not known. |
| 6 | Linux networking | Listen/accept queue overflow; Recv-Q vs Send-Q; SCTP vs TCP | **Weak / partial.** Correct direction: Recv-Q = application slow, Send-Q = outbound side (missed that a slow/zero-window peer also causes it). Named SCTP multihoming. Not known: SYN and accept queues, `somaxconn`, `ListenOverflows`/`ListenDrops`, fd limits, conntrack; `ss -tni` diagnosis; SCTP multi-streaming, message boundaries, cookie handshake. |
| 7 | Performance optimization | Verifying a flamegraph + fair A/B; allocation/copy removal in an encoder (grouped AVP lengths); false sharing of per-thread atomics | **Not known (self-reported).** Learner states no prior performance-optimization experience. Track G must start from methodology: measure, profile, change one thing, re-measure. |
| 8 | Load testing | Average vs percentiles; open vs closed loop under a stall; past experience | **Partial.** Correct: p99 matters more than the average. Not known: open- vs closed-loop, coordinated omission, reporting offered vs achieved rate, errors/timeouts/retransmissions. Experience: has used SIPp (details not given). |
| 9 | Distributed systems | Partition in active/standby; prevention; sync vs async replication; HA experience | **Good conceptually.** Correct: split brain, double charging, fencing, sync = latency cost, async = updates lost on failover. Missing: how fencing works (quorum/witness, epoch tokens, STONITH), quota overspend, sync replication's availability cost. No hands-on HA experience (self-reported). |
| 10 | Binary protocol parsing | Diameter header layout; TCP partial reads; malicious length fields | **Partial.** Knew there are 4-byte Application-ID, Hop-by-Hop and End-to-End fields; swapped Version (1 B) and Length (3 B); missed Command Flags and Command Code. Framing logic correct (accumulate to 20 B, read length, accumulate to length); did not mention keeping leftover bytes for the next message. Malicious-length handling: not answered. |

**Strengths identified:**
- Application-level IMS/Diameter: I-CSCF role, UAR/MAR/SAR purpose, watchdog-based failure detection, peer failover.
- Distributed-systems concepts: split brain, fencing, sync vs async replication trade-offs.
- Correct instincts: p99 over average; length-prefixed framing; CUPS scaling and edge placement.
- Practical tool experience: SIPp.
- Honest self-assessment ("don't know" when not known).

**Gaps identified (priority order for year 1):**
1. Applied Rust ownership/lifetimes (returning borrows of locals; owned vs borrowed vs `Bytes`).
2. Async/Tokio model: tasks vs threads, blocking in async, locks across `.await`, `Send + 'static`.
3. Performance methodology: no prior experience (benchmarks, profiling, allocation, false sharing).
4. Load-testing models: open vs closed loop, coordinated omission.
5. Linux network diagnosis: accept/SYN queues, `ss -tni`, `nstat` counters.
6. Wire-format detail: Diameter header layout, T flag / End-to-End ID duplicate handling.
7. 5G procedures (registration step by step), AKA mechanics (AUTN, MAC, SQN), PFCP.

## Completed tasks

| Task ID | Title | Completed | Evidence |
|---------|-------|-----------|----------|
| ASSESS-000 | 10-question knowledge assessment | 2026-11-16 | Assessment log above |

## Concepts understood

_(only concepts demonstrated in answers, code, or reviews)_

- CUPS motivation: independent scaling, user-plane edge placement (assessment Q1)
- IMS registration Cx roles: UAR at I-CSCF, MAR/SAR at S-CSCF (Q3)
- Diameter watchdog and peer failover, at the concept level (Q3)
- Split brain and its charging impact; fencing; replication latency vs data loss (Q9)
- Tail latency matters more than the average (Q8)

## Rust concepts practiced

_(none yet)_

## Tests

| Task | Passed | Failed | Notes |
|------|-------:|-------:|-------|

## Benchmark results

_No benchmarks run yet. Every entry must include machine specs, rustc version and build profile._

## Performance improvements

_(before/after entries)_

## Known limitations

_(none yet)_

## GitHub milestones

- [x] Repository published — 2026-11-16, https://github.com/rizwan3659/rust-telecom-lab (public)
- [ ] M1 — first public crate with benches

## YouTube tutorials

_(none yet)_

## Next recommended tasks

1. **DIA-HDR-001** — Diameter header codec (current)
2. DIA-AVP-001 — zero-copy AVP iterator
3. DIA-AVP-002 — fuzzing + negative tests
4. PERF-BENCH-001 — first Criterion benchmark

Weeks 1–4 plan: see ROADMAP.md "Weeks 1–4 in detail".

## Environment

- OS: Linux 6.8.0-138-generic
- Toolchain: rustc 1.98.1, cargo 1.98.1
