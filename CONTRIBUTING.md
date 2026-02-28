# Contributing to FemtoClaw Talons

Thank you for your interest in contributing Talons to the FemtoClaw community!

## How to Contribute

### 1. Fork and Clone

```bash
git clone https://github.com/your-username/femtoclaw-talons.git
cd femtoclaw-talons
```

### 2. Create Your Talon

Create a new directory with your talon name:

```bash
mkdir my-talon
cd my-talon
```

### 3. Add TALON.md

Create a `TALON.md` file following the format:

```yaml
---
name: my-talon
version: 1.0.0
description: A brief description of what your talon does
author: Your Name
license: MIT
tags: [category, keywords]
---

# My Talon

Detailed description of your talon.

## Requirements
- List any requirements

## Usage
How to use your talon.
```

### 4. Add Supporting Files

Add any necessary files:
- Source code in `src/`
- Scripts in `scripts/`
- Config templates in `config/`

### 5. Test Locally

```bash
# Install femtoclaw-registry
cargo install femtoclaw-registry

# Add your talon
talon add ./my-talon

# Verify
talon list
```

### 6. Submit a Pull Request

1. Push to your fork
2. Create a pull request
3. Wait for review

## Talon Requirements

### Must Have
- Valid `TALON.md` with required fields
- Clear documentation
- Working basic functionality

### Should Have
- Example usage
- Error handling
- Configuration options

### Nice to Have
- Tests
- CI/CD configuration
- Version upgrades

## Code Style

- Use clear, descriptive names
- Comment complex logic
- Follow Rust conventions for source files

## License

By contributing, you agree that your contributions will be licensed under Apache 2.0.

## Questions?

Open an issue for questions about contributing.
