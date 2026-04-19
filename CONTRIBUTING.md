# Contributing to FemtoClaw Skills

Thank you for your interest in contributing Skills to the FemtoClaw community!

## How to Contribute

### 1. Fork and Clone

```bash
git clone https://github.com/your-username/femtoclaw-skills.git
cd femtoclaw-skills
```

### 2. Create Your Skill

Create a new directory with your skill name:

```bash
mkdir my-skill
cd my-skill
```

### 3. Add SKILL.md

Create a `SKILL.md` file following the format:

```yaml
---
name: my-skill
version: 1.0.0
description: A brief description of what your skill does
author: Your Name
license: MIT
tags: [category, keywords]
---

# My Skill

Detailed description of your skill.

## Requirements
- List any requirements

## Usage
How to use your skill.
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

# Add your skill
skill add ./my-skill

# Verify
skill list
```

### 6. Submit a Pull Request

1. Push to your fork
2. Create a pull request
3. Wait for review

## Skill Requirements

### Must Have
- Valid `SKILL.md` with required fields
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
