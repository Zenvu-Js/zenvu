# Zenvu.js Release & Versioning Policy

To ensure stability for enterprise applications, Zenvu.js strictly follows **Semantic Versioning (SemVer) 2.0.0**.

## 1. Compatibility Guarantees (SemVer)
- **MAJOR (`v2.0.0`)**: Breaking changes. (e.g., redesigning a core API). Migration guides and CLI tools (`Zenvu migrate`) are always provided.
- **MINOR (`v1.2.0`)**: New backward-compatible features. (e.g., adding a new CLI command or hook).
- **PATCH (`v1.1.5`)**: Backward-compatible bug fixes and security patches.

## 2. Deprecation Strategy
Zenvu.js will NEVER immediately remove an API.
1. When an API is slated for removal, it will first be marked with `@deprecated` in the TypeScript typings.
2. The Rust compiler (`Zenvu build`) will emit a deprecation warning in the terminal.
3. The API will remain functional for at least **one full MAJOR release cycle** (e.g., deprecated in v1.4.0, removed in v2.0.0).

## 3. Release Cycle
- **Stable Releases**: Cut every 2 months.
- **LTS (Long Term Support)**: An LTS version is designated every 12 months and receives critical security patches for 24 months.
- **Canary/Nightly**: Automatically built from the `main` branch via GitHub Actions for early adopters.

## 4. RFC Process
Before any MAJOR or MINOR feature is merged, it must go through the Request For Comments (RFC) process. 
See the `RFCs/0000-template.md` for guidelines on submitting an architectural proposal.

## 5. Changelog Automation
Our GitHub Actions pipeline automatically generates `CHANGELOG.md` based on Conventional Commits (`feat:`, `fix:`, `chore:`).
