# @opaque-auth/guessrank-core

This is the core evaluation engine for the OpaqueAuth project, written entirely in pure Rust.

It handles the heavy mathematical lifting for password evaluation, including spatial pattern detection, leetspeak normalization (RLS), and probabilistic scoring.

This crate is compiled to WebAssembly (Wasm) for fast client-side evaluation, and can also be consumed natively by our backend microservices to ensure exact scoring parity.

## Development & Compilation

To work on this crate, you will need standard Rust tooling and `wasm-pack`.

### 1. Native Rust Build
To ensure the crate compiles for standard backend environments (like a Go or Rust server), run:
```bash
cargo build
```

### 2. WebAssembly Build
To compile the core engine into WebAssembly for use in the frontend TypeScript wrapper, use `wasm-pack`. We target the `bundler` environment so it can be seamlessly integrated by standard web bundlers (Vite, Webpack, etc.).

```bash
wasm-pack build --target bundler
```
*Note: A successful Wasm build will generate a `pkg/` directory containing the `.wasm` binary and the auto-generated JavaScript bindings. This `pkg/` folder is `.gitignore`'d by default.*

### 3. Testing
All mathematical models and heuristic algorithms must maintain strict test coverage. To run the native Rust unit and integration tests:

```bash
cargo test
```

## Core Modules (Planned)
- `sanitizater.rs`: Strips padding and normalizes inputs.
- `leetspeak.rs`: Reverses common substitutions (e.g., `@` -> `a`).
- `spatial.rs`: Calculates physical keyboard adjacency (QWERTY/AZERTY).
- `scoring.rs`: Fuses heuristic and probabilistic models into a final GuessRank.
