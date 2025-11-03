# Contributing to Motherfudder Crypter

Thank you for considering contributing to Motherfudder Crypter. This document provides guidelines and instructions for contributing to the project.

---

## Table of Contents

1. [Code of Conduct](#code-of-conduct)
2. [How to Contribute](#how-to-contribute)
3. [Development Setup](#development-setup)
4. [Coding Standards](#coding-standards)
5. [Submission Guidelines](#submission-guidelines)
6. [Testing](#testing)
7. [Documentation](#documentation)

---

## Code of Conduct

This project adheres to professional standards of conduct:

- **Respectful Communication**: Treat all contributors with respect
- **Constructive Feedback**: Provide helpful, actionable feedback
- **Ethical Use**: Contributions must support legitimate security testing
- **No Malicious Code**: All contributions must be for defensive/educational purposes

---

## How to Contribute

### Types of Contributions

We welcome:

- **Bug Reports**: Identify issues with clear reproduction steps
- **Feature Requests**: Propose new capabilities with use cases
- **Code Contributions**: Submit fixes or enhancements
- **Documentation**: Improve or expand existing documentation
- **Testing**: Validate functionality across environments

### Before Contributing

1. Check existing issues and pull requests
2. Ensure contribution aligns with project goals
3. Verify compliance with legal and ethical guidelines
4. Test changes thoroughly before submission

---

## Development Setup

### Prerequisites

- Rust toolchain (1.70.0+)
- .NET SDK 6.0+
- MSBuild
- OpenSSL development libraries
- Git

### Initial Setup

```bash
# Clone repository
git clone <repository-url>
cd MfBuilder

# Install dependencies
cargo build

# Run tests
cargo test

# Build release
cargo build --release
```

### Environment Configuration

```bash
# For bot development
cp .env.example .env
# Add test bot token to .env
```

---

## Coding Standards

### Rust Code

#### Formatting

Use `rustfmt` for consistent formatting:

```bash
cargo fmt --all
```

#### Linting

Run `clippy` before submitting:

```bash
cargo clippy --all-targets --all-features
```

#### Style Guidelines

- Follow Rust naming conventions
- Use descriptive variable names
- Add comments for complex logic
- Keep functions focused and concise
- Handle errors explicitly (avoid `unwrap()` in production code)

Example:

```rust
// Good
pub fn process_binary(path: &Path) -> Result<BinaryArch, Error> {
    let data = fs::read(path)?;
    BinaryArch::determine(&data)
}

// Avoid
pub fn process_binary(path: &Path) -> BinaryArch {
    let data = fs::read(path).unwrap();
    BinaryArch::determine(&data)
}
```

### C# Code

#### Style

- Follow Microsoft C# coding conventions
- Use PascalCase for public members
- Use camelCase for local variables
- Add XML documentation for public APIs

Example:

```csharp
/// <summary>
/// Patches AMSI scanning functionality
/// </summary>
/// <returns>True if patch successful</returns>
public static bool PatchAMSI()
{
    // Implementation
}
```

### Documentation

- Update relevant `.md` files with changes
- Add inline code comments where necessary
- Include examples for new features
- Keep documentation clear and concise

---

## Submission Guidelines

### Branch Naming

Use descriptive branch names:

- `feature/description` - New features
- `fix/description` - Bug fixes
- `docs/description` - Documentation updates
- `refactor/description` - Code refactoring

Examples:
- `feature/improve-amsi-bypass`
- `fix/bot-authentication-error`
- `docs/update-installation-guide`

### Commit Messages

Write clear, descriptive commit messages:

```
Short summary (50 chars or less)

Detailed explanation if needed. Wrap at 72 characters.

- Bullet points for multiple changes
- Reference issue numbers: #123
```

Examples:

```
Add rate limiting to Telegram bot

Implements basic rate limiting to prevent abuse. Users are
limited to one request per 5 seconds.

- Add RateLimit struct
- Implement check method
- Update message_handler
```

### Pull Requests

1. **Create Branch**: Branch from `main`

```bash
git checkout -b feature/my-new-feature
```

2. **Make Changes**: Implement your contribution

3. **Test Thoroughly**: Ensure all tests pass

```bash
cargo test
cargo build --release
# Test manually if needed
```

4. **Commit**: Use clear commit messages

```bash
git add .
git commit -m "Add feature description"
```

5. **Push**: Push to your fork

```bash
git push origin feature/my-new-feature
```

6. **Open PR**: Create pull request with description

**PR Template**:

```markdown
## Description
Brief description of changes

## Type of Change
- [ ] Bug fix
- [ ] New feature
- [ ] Documentation update
- [ ] Refactoring

## Testing
Description of testing performed

## Checklist
- [ ] Code follows project style guidelines
- [ ] Self-review completed
- [ ] Comments added where necessary
- [ ] Documentation updated
- [ ] Tests pass
- [ ] No new warnings
```

---

## Testing

### Running Tests

```bash
# All tests
cargo test

# Specific module
cargo test telegram_bot

# With output
cargo test -- --nocapture
```

### Manual Testing

For bot functionality:

1. Set up test bot with @BotFather
2. Configure test token in `.env`
3. Run bot: `cargo run -- --bot`
4. Test all commands and workflows
5. Verify error handling

For CLI mode:

1. Create test configuration in `build.json`
2. Prepare test payload
3. Run build: `cargo run`
4. Verify output file
5. Test edge cases

### Test Coverage

Aim for comprehensive coverage:
- Happy path scenarios
- Error conditions
- Edge cases
- Invalid inputs
- Concurrent operations (for bot)

---

## Documentation

### Types of Documentation

1. **Code Comments**: Explain complex logic
2. **API Documentation**: Rust doc comments
3. **User Guides**: Markdown files in `docs/`
4. **README**: Keep updated with new features

### Documentation Standards

- Use clear, concise language
- Provide examples where helpful
- Keep formatting consistent
- Update table of contents as needed
- Include relevant links

### Building Documentation

```bash
# Rust documentation
cargo doc --open

# This generates HTML docs from source comments
```

---

## Review Process

### What Reviewers Look For

- **Functionality**: Does it work as intended?
- **Code Quality**: Is it well-written and maintainable?
- **Testing**: Is it adequately tested?
- **Documentation**: Is it properly documented?
- **Security**: Does it introduce vulnerabilities?
- **Performance**: Is it efficient?

### Addressing Feedback

- Respond promptly to review comments
- Make requested changes in new commits
- Ask for clarification if needed
- Be open to suggestions
- Update PR description if scope changes

---

## Feature Development Guidelines

### New Features

Before implementing major features:

1. Open an issue for discussion
2. Describe use case and benefits
3. Outline implementation approach
4. Get maintainer feedback
5. Proceed with implementation

### Breaking Changes

If your change breaks existing functionality:

1. Clearly mark as breaking change
2. Provide migration guide
3. Update version appropriately
4. Document in CHANGELOG.md

---

## Security Considerations

### Reporting Vulnerabilities

If you discover a security vulnerability:

1. **DO NOT** open a public issue
2. Contact maintainers privately
3. Provide detailed description
4. Include reproduction steps if possible
5. Allow reasonable time for fix before disclosure

### Security in Contributions

- Never commit secrets or tokens
- Validate all user inputs
- Handle errors securely
- Consider attack surface
- Follow secure coding practices

---

## Legal and Ethical Guidelines

### Acceptable Contributions

- Enhancements for legitimate security testing
- Bug fixes and stability improvements
- Performance optimizations
- Documentation improvements
- Educational features

### Unacceptable Contributions

- Features designed for malicious use
- Circumventing legal protections
- Violating terms of service
- Enabling unauthorized access
- Facilitating illegal activities

---

## Recognition

Contributors will be recognized in:
- CHANGELOG.md for significant contributions
- Git commit history
- Release notes

---

## Questions?

- Open an issue for general questions
- Tag maintainers for specific guidance
- Review existing documentation first
- Check closed issues for previous discussions

---

## Additional Resources

- [Rust Book](https://doc.rust-lang.org/book/)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [C# Coding Conventions](https://docs.microsoft.com/en-us/dotnet/csharp/fundamentals/coding-style/coding-conventions)
- [Conventional Commits](https://www.conventionalcommits.org/)

---

Thank you for contributing to Motherfudder Crypter!
