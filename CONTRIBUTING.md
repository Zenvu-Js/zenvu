# Contributing to Zenvu.js ðŸ”µ

First off, thank you for considering contributing to Zenvu.js! It's people like you that make Zenvu.js the most advanced meta-framework in the world.

## 1. Where do I go from here?

If you've noticed a bug or have a feature request, make one! It's generally best if you get confirmation of your bug or approval for your feature request this way before starting to code.
If you have a massive architectural change, please refer to the `RFC` templates in `.github/ISSUE_TEMPLATE`.

## 2. Setting up your local environment

1. **Rust Toolchain**: You need to install Rust (via rustup) to compile the core engines.
2. **Node.js**: You need Node 20+ to build the TypeScript runtime packages.
3. Fork and clone the repository.
4. Run `cargo build --workspace` to ensure the Rust core compiles.
5. Run `npm install` and `npm run build` in the `packages/` directory.

## 3. Architecture Rules
- **No Virtual DOM**: Do not introduce any diffing logic in the runtime. All reactivity must use the proxy-based Signal DAG.
- **Strict Memory Quotas**: Any addition to the client runtime must not increase the overall payload size above our strict 20KB limit.
- **Memory Safety**: Do not use `unsafe` blocks in Rust unless absolutely required for FFI (V8 Sandbox bindings).

## 4. Pull Requests
1. Create a new branch: `git checkout -b my-feature-branch`.
2. Make your changes and commit them following the Conventional Commits specification (e.g., `feat(compiler): added new XYZ parser`).
3. Push to your fork and submit a Pull Request.
4. Ensure the GitHub Actions CI pipeline passes (Memory Safety & Linting).

Welcome to the cutting edge of web development!
