## About

**liboqs-rust Maintainer — Open Quantum Safe (OQS) | Rust Systems Engineer | Applied Cryptography | Distributed & Autonomous Systems**

I am a maintainer of liboqs-rust and a member of Open Quantum Safe (OQS), serving on both the liboqs-rust maintainers and release managers teams.

**Maintainer appointment**: Governance PR #362 — Add Orlando as liboqs-rust Maintainer [vote]
I build secure, reliability-critical infrastructure in Rust, with current work spanning post-quantum cryptography, distributed systems, embedded/aerospace software, and portable runtimes.

I am developing **Quantum-Safe Agentic Infrastructure** — a practical architecture for securing long-lived, autonomous, and AI-driven systems in a post-quantum environment.

**Current focus**
- Applied post-quantum cryptography and secure key infrastructure
- liboqs-rust maintainership and cryptographic interoperability
- Rust systems and runtime engineering
- Distributed systems reliability
- Embedded, aerospace, and autonomous systems
- Portable cryptographic acceleration and secure execution

Manifesto: [quantum-safe-agentic-infrastructure](https://github.com/0rlych1kk4/quantum-safe-agentic-infrastructure)

Architecture & threat model: [quantum-vault-architecture](https://github.com/Orlych1kk4/quantum-vault-architecture)


[![Crates.io](https://img.shields.io/crates/v/solana-validator-optimizer.svg)](https://crates.io/crates/solana-validator-optimizer)
[![Docs.rs](https://docs.rs/solana-validator-optimizer/badge.svg)](https://docs.rs/solana-validator-optimizer)

Systems and security engineer with 9+ years of experience across infrastructure, embedded systems, security, and performance-sensitive software, with a current focus on Rust and post-quantum systems.

---

## Selected Upstream Engineering Impact

### Open Quantum Safe — Post-Quantum Cryptography

Contributed across liboqs, liboqs-rust, and the QR-UOV integration repository, covering post-quantum signature integration, shared cryptographic backends, Rust API exposure, metadata generation, portability, and importer reliability.

**Selected upstream contributions:**

- **liboqs PR #2582** — Add QR-UOV signature schemes
- **liboqs PR #2642** — Use the common AES backend for QR-UOV
- **liboqs PR #2643** — Deduplicate QR-UOV signature wrappers
- **liboqs PR #2654** — Fix importer handling for empty SIG/KEM implementation targets
- **liboqs-rust PR #310** — Implement RustCrypto traits for signatures
- **liboqs-rust PR #312** — Expose padded Falcon signature algorithms
- **liboqs-rust PR #313** — Generate signature algorithm metadata from liboqs
- **qruov/round3-liboqs PR #1** — Add liboqs integration support for QR-UOV

**Engineering focus:** Post-quantum cryptography, C/Rust interoperability, cryptographic integration, API design, code generation, portability, and cross-platform reliability.

### QGroundControl / PX4 — MAVLink Flight-Mode Reliability

**Merged upstream:** [QGroundControl PR #15243](https://github.com/mavlink/qgroundcontrol/pull/15243)

Implemented a fix for MAVLink `AVAILABLE_MODES` enumeration that restarts the download when the response sequence changes, preventing inconsistent flight-mode lists.

**Independent cross-project acknowledgment:** [PX4-Autopilot Issue #28936](https://github.com/PX4/PX4-Autopilot/issues/28936)

A PX4 contributor subsequently referenced PR #15243 as the fix when closing a related issue involving unexpected flight-mode behavior following a soft reboot.

**Engineering focus:** C++, MAVLink protocol correctness, asynchronous state handling, and aerospace ground-control software.

### Space & Embedded Rust — Flight and Protocol Correctness

Contributed upstream fixes across sat-rs, spacepackets-rs, and vorago-rs, covering spacecraft subsystem sequencing, CCSDS packet calculations, CUC time rollover, PLL operating bounds, and flashloader input validation.

**Merged upstream:**

- **sat-rs — PR #4:** correct CCSDS packet length calculation
- **sat-rs — PR #5:** reject sequence tables exceeding u8 capacity
- **spacepackets-rs — PR #15:** correct CUC counter rollover modulus
- **vorago-rs — PR #4:** reject short flashloader memory-write payloads
- **vorago-rs — PR #5:** enforce exact PLL VCO bounds

**Engineering focus:** Rust, spacecraft software, CCSDS protocol correctness, embedded systems, boundary-condition validation, and hardware-facing reliability.

### MAVLink / Rust — Performance & CI Engineering

**Merged upstream:** [rust-mavlink PR #528](https://github.com/mavlink/rust-mavlink/pull/528)

Implemented CI-based performance regression tooling for `rust-mavlink`, comparing pull-request builds against the base revision for compilation time and `libmavlink` artifact size.

The implementation includes reusable local tooling and GitHub Actions integration, allowing performance measurements to be reproduced outside CI while keeping the workflow itself lightweight.

**Engineering focus:** Rust, MAVLink, CI/CD, build-performance analysis, artifact-size regression tracking, and developer infrastructure.

### Ericsson ecChronos — Distributed Systems Reliability
Contributed upstream fixes addressing scheduler concurrency, saturation backoff, and distributed scheduling behavior.

**Selected contributions:**
- [PR #1861 — Saturation backoff overflow](https://github.com/Ericsson/ecchronos/pull/1861)
- [PR #1859 — Scheduler concurrency after node removal](https://github.com/Ericsson/ecchronos/pull/1859)

**Engineering focus:** Distributed scheduling, concurrency correctness, reliability, and fault handling.

---

## Upstream Open-Source Contributions

I contribute to systems projects where correctness, portability, security, and reliability matter.

- **Open Quantum Safe / liboqs-rust** — post-quantum cryptography, Rust/C interoperability, build portability, CI, and platform-specific issues
- **rust-mavlink** — MAVLink protocol tooling and aerospace/telemetry systems in Rust
- **sat-rs** — spacecraft and embedded Rust software
- **Ericsson ecChronos** — distributed scheduling, repair, and reliability
- **WASM/runtime projects** — portable and sandboxed systems execution


---

## Merged Open-Source Contributions

<!-- MERGED-PRS:START -->
**102 merged pull requests across 14 external repositories.** Automatically generated from my upstream GitHub contributions.

### [Ericsson/codechecker](https://github.com/Ericsson/codechecker) — 1 merged PR

| PR | Contribution |
| --- | --- |
| [#4777](https://github.com/Ericsson/codechecker/pull/4777) | docker: add container HEALTHCHECK |

### [Ericsson/ecchronos](https://github.com/Ericsson/ecchronos) — 11 merged PRs

| PR | Contribution |
| --- | --- |
| [#1861](https://github.com/Ericsson/ecchronos/pull/1861) | Fix saturation backoff overflow after prolonged client saturation |
| [#1859](https://github.com/Ericsson/ecchronos/pull/1859) | Fix scheduler concurrency cap after node removal |
| [#1769](https://github.com/Ericsson/ecchronos/pull/1769) | test: verify node registration before scheduling |
| [#1742](https://github.com/Ericsson/ecchronos/pull/1742) | test: add ConfigurationHelper regression coverage |
| [#1708](https://github.com/Ericsson/ecchronos/pull/1708) | test: cover successful scheduled job not rerun before interval |

[View all 11 merged PRs in Ericsson/ecchronos](https://github.com/Ericsson/ecchronos/pulls?q=is%3Apr+is%3Amerged+author%3A0rlych1kk4)

### [hansonp303/rust_checker](https://github.com/hansonp303/rust_checker) — 27 merged PRs

| PR | Contribution |
| --- | --- |
| [#32](https://github.com/hansonp303/rust_checker/pull/32) | fix(lib): expose configuration module |
| [#31](https://github.com/hansonp303/rust_checker/pull/31) | fix(cargo): resolve duplicate manifest keys |
| [#30](https://github.com/hansonp303/rust_checker/pull/30) | ci: migrate artifact actions to v4 |
| [#29](https://github.com/hansonp303/rust_checker/pull/29) | chore: ignore macOS metadata files |
| [#28](https://github.com/hansonp303/rust_checker/pull/28) | Feature/junit xml export |

[View all 27 merged PRs in hansonp303/rust_checker](https://github.com/hansonp303/rust_checker/pulls?q=is%3Apr+is%3Amerged+author%3A0rlych1kk4)

### [malekchule/nanobot_control_system](https://github.com/malekchule/nanobot_control_system) — 38 merged PRs

| PR | Contribution |
| --- | --- |
| [#41](https://github.com/malekchule/nanobot_control_system/pull/41) | fix(task-manager): clarify whitespace errors in task IDs |
| [#40](https://github.com/malekchule/nanobot_control_system/pull/40) | fix(task-manager): reject blank loaded task descriptions |
| [#39](https://github.com/malekchule/nanobot_control_system/pull/39) | fix(task-manager): reject blank task descriptions |
| [#38](https://github.com/malekchule/nanobot_control_system/pull/38) | test(task-manager): clean up temporary files after panics |
| [#37](https://github.com/malekchule/nanobot_control_system/pull/37) | fix(task-manager): propagate task file flush errors |

[View all 38 merged PRs in malekchule/nanobot_control_system](https://github.com/malekchule/nanobot_control_system/pulls?q=is%3Apr+is%3Amerged+author%3A0rlych1kk4)

### [mavlink/qgroundcontrol](https://github.com/mavlink/qgroundcontrol) — 1 merged PR

| PR | Contribution |
| --- | --- |
| [#15243](https://github.com/mavlink/qgroundcontrol/pull/15243) | Fix AVAILABLE_MODES download when sequence changes |

### [mavlink/rust-mavlink](https://github.com/mavlink/rust-mavlink) — 3 merged PRs

| PR | Contribution |
| --- | --- |
| [#528](https://github.com/mavlink/rust-mavlink/pull/528) | feat: add profiling support for MAVLink performance analysis |
| [#521](https://github.com/mavlink/rust-mavlink/pull/521) | fix: decode XML entities in generated rustdoc |
| [#520](https://github.com/mavlink/rust-mavlink/pull/520) | docs: add frame constants usage example |

### [nebari-dev/nebari-infrastructure-core](https://github.com/nebari-dev/nebari-infrastructure-core) — 2 merged PRs

| PR | Contribution |
| --- | --- |
| [#121](https://github.com/nebari-dev/nebari-infrastructure-core/pull/121) | CI: Automatically regenerate OpenTofu lockfiles for provider templates |
| [#89](https://github.com/nebari-dev/nebari-infrastructure-core/pull/89) | test: add unit tests for GetKubeconfig (fixes #9) |

### [open-quantum-safe/liboqs](https://github.com/open-quantum-safe/liboqs) — 5 merged PRs

| PR | Contribution |
| --- | --- |
| [#2654](https://github.com/open-quantum-safe/liboqs/pull/2654) | fix(importer): skip empty SIG and KEM implementation targets |
| [#2643](https://github.com/open-quantum-safe/liboqs/pull/2643) | refactor(qruov): deduplicate signature wrappers across implementations |
| [#2642](https://github.com/open-quantum-safe/liboqs/pull/2642) | refactor(qruov): use liboqs common AES backend |
| [#2582](https://github.com/open-quantum-safe/liboqs/pull/2582) | Add QR-UOV signature schemes |
| [#2508](https://github.com/open-quantum-safe/liboqs/pull/2508) | Fix ARM SHA2 build with AppleClang on macOS |

### [open-quantum-safe/liboqs-rust](https://github.com/open-quantum-safe/liboqs-rust) — 6 merged PRs

| PR | Contribution |
| --- | --- |
| [#314](https://github.com/open-quantum-safe/liboqs-rust/pull/314) | docs: explain signature generation exclusions |
| [#313](https://github.com/open-quantum-safe/liboqs-rust/pull/313) | refactor: generate signature algorithm metadata from liboqs |
| [#312](https://github.com/open-quantum-safe/liboqs-rust/pull/312) | feat: expose padded Falcon signature algorithms |
| [#310](https://github.com/open-quantum-safe/liboqs-rust/pull/310) | feat: implement RustCrypto traits for signatures |
| [#309](https://github.com/open-quantum-safe/liboqs-rust/pull/309) | fix: update wrapper for latest liboqs algorithm names |

[View all 6 merged PRs in open-quantum-safe/liboqs-rust](https://github.com/open-quantum-safe/liboqs-rust/pulls?q=is%3Apr+is%3Amerged+author%3A0rlych1kk4)

### [qruov/round3-liboqs](https://github.com/qruov/round3-liboqs) — 1 merged PR

| PR | Contribution |
| --- | --- |
| [#1](https://github.com/qruov/round3-liboqs/pull/1) | Add liboqs integration support for QR-UOV |

### [tokio-rs/tokio](https://github.com/tokio-rs/tokio) — 1 merged PR

| PR | Contribution |
| --- | --- |
| [#7923](https://github.com/tokio-rs/tokio/pull/7923) | task: clarify when to use spawn_blocking vs dedicated threads |

### [ustgt-irs/sat-rs](https://github.com/ustgt-irs/sat-rs) — 3 merged PRs

| PR | Contribution |
| --- | --- |
| [#6](https://github.com/ustgt-irs/sat-rs/pull/6) | fix Clippy 1.99 redundant field name lint |
| [#5](https://github.com/ustgt-irs/sat-rs/pull/5) | subsystem: reject sequence tables exceeding u8 capacity |
| [#4](https://github.com/ustgt-irs/sat-rs/pull/4) | fix: correct CCSDS packet length calculation |

### [ustgt-irs/spacepackets-rs](https://github.com/ustgt-irs/spacepackets-rs) — 1 merged PR

| PR | Contribution |
| --- | --- |
| [#15](https://github.com/ustgt-irs/spacepackets-rs/pull/15) | fix(time): correct CUC counter rollover modulus |

### [ustgt-irs/vorago-rs](https://github.com/ustgt-irs/vorago-rs) — 2 merged PRs

| PR | Contribution |
| --- | --- |
| [#5](https://github.com/ustgt-irs/vorago-rs/pull/5) | fix: enforce exact PLL VCO bounds |
| [#4](https://github.com/ustgt-irs/vorago-rs/pull/4) | flashloader: reject short memory write payloads |

<!-- MERGED-PRS:END -->

---

## Featured Crates & Projects

### `quantum-safe-agentic-infrastructure`

A manifesto and reference framework for securing AI-driven, post-quantum, and distributed digital infrastructure using Rust, PQC, agentic AI security, and reproducible systems engineering.

> Connects my work across post-quantum cryptography, Quantum Vault, agentic AI security labs, blast-radius evaluation, reproducible quantum compilation, and distributed systems reliability.  
> [View the manifesto](https://github.com/0rlych1kk4/quantum-safe-agentic-infrastructure)

### `solana-validator-optimizer`

Optimizes Solana validator performance with snapshot prefetching, RPC caching, and Prometheus metrics.

> 450+ downloads in 2 weeks | MIT Licensed | Production-ready  
> [View on crates.io](https://crates.io/crates/solana-validator-optimizer)

### `quantum_hsm`

Hardware Security Module simulation and integration for quantum-resilient key storage and signing.

### `ai_nanotech_sim`

AI-powered nanotech simulation in Rust using ML and asynchronous modeling of particle interactions.

### `quantum_cryptanalysis`

AI + quantum + classical analysis for testing and breaking post-quantum schemes for academic and defensive research.

### `quantum_safe_multisig`

Multi-signature wallet with SPHINCS+, quantum-resistant HSM signing, and hybrid encryption.

> Many of these are [MIT-licensed and open for contributions](https://github.com/0rlych1kk4)

---

## Engineering Highlights

- Contributor to upstream Rust, cryptography, aerospace, and distributed-systems projects
- Author of multiple public Rust crates
- Building practical post-quantum security and key-management infrastructure
- Developing secure runtime and autonomous-systems architecture in Rust

---

## Open to Collaboration & Opportunities

Open to international engineering opportunities and research collaborations in:

- Applied cryptography and post-quantum security
- Rust systems and infrastructure engineering
- Secure communications and cryptographic protocols
- Distributed systems and reliability engineering
- Embedded, aerospace, and autonomous systems

Available for remote collaboration and employer-sponsored international relocation, particularly in Germany, Switzerland, and the Netherlands.

Interested in building secure, reliable, and deployable infrastructure that addresses real-world security and emerging technology challenges.

Email: `orlychikka@gmail.com`  
LinkedIn: (https://linkedin.com/in/orlando-t-3893416b?)  
GitHub Discussions: enabled on select projects

---

Thanks for stopping by. Feel free to star or fork any project you find helpful!
