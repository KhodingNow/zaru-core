
### 7. Create `CHANGELOG.md`

```markdown
# Changelog

All notable changes to this project will be documented in this file.

## [0.1.0] - 2024-07-22

### Added
- Initial release
- Amount type with safe construction
- Transaction state machine (Unsigned → Signed → Verified)
- ED25519 cryptographic verification
- Wallet abstraction (Bank + Crypto)
- Python bindings with PyO3
- Async support with Tokio
- Comprehensive test suite
- Documentation

### Security
- Secure random key generation
- Signature verification
- No unsafe code
