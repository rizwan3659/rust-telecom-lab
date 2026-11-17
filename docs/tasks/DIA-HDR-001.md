# DIA-HDR-001 — Diameter message header codec

| | |
|---|---|
| **Task ID** | DIA-HDR-001 |
| **Title** | Decode, validate and encode the 20-byte Diameter message header |
| **Difficulty** | Beginner–Intermediate (Rust), Beginner (telecom) |
| **Estimated time** | 3–4 h (including workspace setup) |
| **Prerequisites** | Rust slices and borrowing, `Result`, enums, `u32::from_be_bytes` |
| **Mode** | **Hand-written foundation task** (CLAUDE.md §16): you write the code; Claude hints and reviews only |
| **Status** | Assigned 2026-11-16 |

## Learning objectives

- Read a binary protocol layout from an RFC and map it exactly to code.
- Parse from a borrowed `&[u8]` **without panicking on any input** and without heap allocation.
- Design an error type that separates "not enough bytes yet" from "invalid message".
- Handle a non-native field size (24-bit integers) and bit flags.
- Write positive, negative, boundary and property-based tests.

## Theory

Every Diameter message starts with a fixed 20-byte header, followed by AVPs. All integers
are big-endian (network byte order).

```
 0                   1                   2                   3
 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|    Version    |                 Message Length                |   bytes 0–3
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
| Command Flags |                  Command Code                 |   bytes 4–7
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|                         Application-ID                        |   bytes 8–11
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|                      Hop-by-Hop Identifier                    |   bytes 12–15
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|                      End-to-End Identifier                    |   bytes 16–19
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
```

| Field | Size | Meaning |
|---|---|---|
| Version | 1 byte | Must be 1. |
| Message Length | 3 bytes | Whole message **including** the header and padded AVPs, so always a multiple of 4. |
| Command Flags | 1 byte | `R` 0x80 request (clear = answer) · `P` 0x40 proxiable · `E` 0x20 error · `T` 0x10 potentially retransmitted · low 4 bits reserved |
| Command Code | 3 bytes | e.g. 257 CER/CEA, 280 DWR/DWA, 282 DPR/DPA; Cx: 300 UAR, 301 SAR, 302 LIR, 303 MAR |
| Application-ID | 4 bytes | 0 = Diameter common messages; 16777216 = 3GPP Cx/Dx |
| Hop-by-Hop Identifier | 4 bytes | Matches answers to requests on one hop; changes at every relay. |
| End-to-End Identifier | 4 bytes | Detects duplicates end to end; unchanged by relays and on retransmission. |

Rules from the RFC this task enforces:

- The `E` bit **must not** be set on a request.
- Reserved flag bits **must be sent as zero and ignored by the receiver**.
- A version other than 1 is answered with `DIAMETER_UNSUPPORTED_VERSION` (5011). Here the
  decoder only reports it; generating answers comes later.

## Standards

- RFC 6733 §3 — Diameter Header
- RFC 6733 §3.1 — Command Codes
- RFC 6733 §7.1.5 — Permanent failures (5011 `DIAMETER_UNSUPPORTED_VERSION`)
- 3GPP TS 29.229 — Cx/Dx command codes and Application-ID (for the test vectors only)

## Architecture

Scope is **only the header**. No AVPs, no sockets, no async.

```
  &[u8] (≥ 20 bytes) ──► decode ──► DiameterHeader  (or HeaderError)
  DiameterHeader     ──► encode ──► 20 bytes written into a caller-provided buffer
```

Later tasks build on this: the AVP iterator (DIA-AVP-001) parses the bytes after the
header, and the TCP framer (NET-FRAME-001) uses `decode` to learn how many bytes the full
message needs. Design your error type with that framer in mind.

## Implementation requirements

1. **Workspace setup**
   - Root `Cargo.toml` with a `[workspace]` containing `crates/*`.
   - Library crate `crates/diameter-codec` (edition 2024).
2. **Types**
   - A `DiameterHeader` struct with one typed field per header field.
   - A command-flags representation with readable accessors (e.g. "is this a request?").
   - A public error enum that implements `std::error::Error` and `Display`.
3. **Decode**, from a borrowed `&[u8]`, with a caller-supplied maximum message length:
   - Fewer than 20 bytes → an "incomplete" error that reports how many bytes are needed.
   - Version ≠ 1 → error carrying the received version.
   - Message Length < 20 → error.
   - Message Length not a multiple of 4 → error.
   - Message Length > configured maximum → error.
   - `E` bit set together with `R` bit → error.
   - Reserved flag bits set → **accepted**; they are ignored when decoding.
   - Decode only the header. Do not require the full message to be in the buffer.
4. **Encode** into a caller-provided buffer, writing exactly 20 bytes:
   - Reserved flag bits always written as 0.
   - Values that cannot be encoded (Message Length or Command Code above 24 bits) must not be
     silently truncated. Choose how to prevent it (types or errors) and document why.
5. **No panics, no heap allocation** in decode or encode. That means no `unwrap`/`expect`,
   no indexing that could go out of bounds, and no `Vec`/`String` in these paths.

## Rust concepts

Slices and borrowing (`&[u8]`), `TryFrom` for slice-to-array conversion, big-endian integer
conversion, newtypes, bit masks as `const`s, enums with data for errors, `#[derive]` choices
(`Copy`? `Eq`? `Hash`?), `Display`/`Error` impls, `cargo clippy` lints. Optional: `thiserror`.
If you use it, explain the trade-off in the README.

## Acceptance criteria

- [ ] `cargo build`, `cargo test`, `cargo clippy --all-targets -- -D warnings` and
      `cargo fmt --check` all pass from the workspace root.
- [ ] The test vectors below decode to exactly the listed field values.
- [ ] Every invalid case in requirement 3 returns its own error variant.
- [ ] Round trip: `encode(decode(b)) == b` for every valid header with reserved bits zero.
- [ ] A property test shows decode never panics on arbitrary input (≥ 10,000 cases).
- [ ] No `unwrap`, `expect`, `panic!` or panicking index in the library code (tests may use them).
- [ ] Crate README: layout diagram, the validation rules with RFC section references, and your design decisions.

## Test vectors

All bytes are hex. Use them as-is in unit tests.

| Name | Bytes | Expected |
|---|---|---|
| CER header | `01 00 00 14 80 00 01 01 00 00 00 00 00 00 00 01 00 00 00 02` | v1, len 20, R, cmd 257, app 0, hbh 1, e2e 2 |
| DWA header | `01 00 00 14 00 00 01 18 00 00 00 00 12 34 56 78 9a bc de f0` | v1, len 20, answer, cmd 280, app 0, hbh 0x12345678, e2e 0x9abcdef0 |
| Cx MAR header | `01 00 01 2c c0 00 01 2f 01 00 00 00 00 00 00 07 00 00 00 08` | v1, len 300, R+P, cmd 303, app 16777216, hbh 7, e2e 8 |

## Testing requirements

| Kind | Cases |
|---|---|
| Positive | the three vectors; each flag individually; T flag on a request |
| Negative | version 0 and 2; length 0, 16, 19; length 22; length above the max; E+R together; 0 bytes and 19 bytes of input |
| Boundary | exactly 20 bytes of input; length = 20; length = configured max; length = 0xFFFFFC with max = 0xFFFFFF; reserved bits set (accepted, and zero after re-encode) |
| Property | random 20-byte arrays and random-length slices: decode never panics; valid headers round-trip |

`proptest` is a reasonable dev-dependency for the property tests.

## Performance requirements

No benchmark in this task. Criterion arrives in PERF-BENCH-001 next week and will measure this
code. The requirement here is structural: **zero heap allocations** in decode and encode.
Explain in the README how your design guarantees this.

## Hints (read one at a time, only when stuck)

<details><summary>Hint 1</summary>
Draw the 20 bytes with byte offsets before writing code. Which fields start at a byte that is
not a multiple of 4, and which ones share a 4-byte word?
</details>

<details><summary>Hint 2</summary>
<code>buf.get(a..b)</code> returns an <code>Option</code> instead of panicking. A slice of known
length converts to a fixed array with <code>try_into()</code>. For a 3-byte field, think about
what array you could build to reuse <code>u32::from_be_bytes</code>.
</details>

<details><summary>Hint 3</summary>
Check that you have at least 20 bytes <em>once</em>, at the start. After that, can you convert
the input into a <code>&amp;[u8; 20]</code> so the rest of the function cannot go out of bounds?
</details>

<details><summary>Hint 4</summary>
For the framer in week 4: "need more bytes" means wait and read again, while "invalid" means
close the connection. If both are the same error variant, how would the framer tell them apart?
</details>

## Review questions

1. Why must "incomplete" be a separate error from "invalid"? What does the TCP framer do with each?
2. The RFC says to reject `E` on a request but ignore reserved bits. Why the different treatment? What would break in the future if receivers rejected reserved bits?
3. How did you represent the 24-bit fields? What invalid values can your types still hold, and where are they caught?
4. Did you derive `Copy` for `DiameterHeader`? Justify it in terms of size and use.
5. A peer on TCP sends a header with Message Length 16. If the framer believed it, what would happen to the parsing of the *next* message? What should happen to the connection?

## Deliverables

- `crates/diameter-codec` source and tests
- Crate `README.md`
- `cargo test` and `cargo clippy` output pasted into `LEARNING_PROGRESS.md` as evidence
- Written answers to the five review questions (in chat or in `docs/`)

## YouTube opportunity

- **Title:** "Parsing Diameter in Rust: 20 bytes that can't crash your network"
- **Demo:** a CER from a Wireshark capture decoded byte by byte; then a malformed length fed in, shown next to what the same code in C would do.
- **Key points:** reading an RFC header diagram; 24-bit fields; incomplete vs invalid; why safe Rust removes a whole class of parser CVEs.
