# Changelog

All notable changes to the Kaspa Python SDK are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [Unreleased]

### Added
- MkDocs documentation site with Material theme
- Comprehensive API reference
- User guides and tutorials

---

## [1.0.1.post2] - Current

### Added
- Full transaction building and signing support
- RPC client with WebSocket support
- HD wallet key derivation (BIP-32/BIP-44)
- Mnemonic seed phrase generation (BIP-39)
- Multi-signature address support
- Message signing and verification
- Script builder for custom transactions

### Features
- Address creation and validation
- UTXO management
- Fee estimation
- Event subscriptions (UTXO changes, blocks, etc.)
- Network resolver for automatic node discovery

---

## Version History

For the complete version history, see the [CHANGELOG.md](https://github.com/your-org/kaspa-python/blob/main/CHANGELOG.md) in the repository.

---

## Upgrading

### From Pre-1.0

If upgrading from a pre-1.0 version:

1. Update import statements - all classes are now under `kaspa`
2. Check for renamed methods or changed signatures
3. Test thoroughly in a development environment

### Breaking Changes

Breaking changes are documented in each version's release notes. Major version bumps indicate breaking changes.

---

## Reporting Issues

Found a bug or have a feature request? Please [open an issue](https://github.com/your-org/kaspa-python/issues) on GitHub.

