# FemtoClaw Talons

**Community-built Talons for FemtoClaw**

A collection of Talons created and maintained by the FemtoClaw Community. Talons extend FemtoClaw's capabilities with additional tools, integrations, and features.

**License:** Apache-2.0  
**Contributing:** See CONTRIBUTING.md

---

## What are Talons?

Talons are self-contained capability extensions for FemtoClaw. Think of them like apps for your phone - they extend what FemtoClaw can do.

### Quick Example

```
my-talon/
├── TALON.md          # Manifest
├── src/              # Source code (optional)
└── scripts/          # Helper scripts (optional)
```

---

## Installing Talons

```bash
# Install talon CLI
cargo install femtoclaw-registry

# Initialize talons directory
talon init

# Add a talon
talon add ./path/to/talon

# List installed talons
talon list
```

---

## Available Talons (11 Categories)

### 1. Example & Templates

| Talon | Description |
|-------|-------------|
| `example` | Template talon for creating new talons |

### 2. Version Control

| Talon | Description |
|-------|-------------|
| `git` | Git operations - commit, push, pull, branch |
| `github` | GitHub integration for issues, PRs, and workflows |

### 3. Containers & Cloud

| Talon | Description |
|-------|-------------|
| `docker` | Docker container management |
| `kubernetes` | Kubernetes pod, deployment, and service management |
| `aws` | AWS operations - EC2, S3, Lambda |

### 4. Communication

| Talon | Description |
|-------|-------------|
| `email` | Email sending via SMTP |
| `slack` | Slack notifications and messaging |

### 5. System & Utilities

| Talon | Description |
|-------|-------------|
| `terminal` | Terminal utilities, process management, system info |
| `search` | Web search, Wikipedia, news search |

### 6. Database (Coming Soon)

| Talon | Description |
|-------|-------------|
| `database` | Database query, backup, and management |

---

## Creating Your Own Talon

### 1. Create the Structure

```bash
mkdir my-awesome-talon
cd my-awesome-talon
```

### 2. Create TALON.md

```yaml
---
name: my-awesome-talon
version: 1.0.0
description: An awesome talon that does something useful
author: Your Name
license: MIT
tags: [awesome, example]
---

# My Awesome Talon

This talon does something useful.

## Commands

### do-something
Does something awesome.

## Requirements
- None

## Usage
This talon can be used to do awesome things.
```

### 3. Add Supporting Files

- `src/` - Rust source code
- `scripts/` - Shell scripts  
- `config/` - Configuration templates

### 4. Test Locally

```bash
talon add ./my-awesome-talon
talon list
```

---

## Directory Structure

```
femtoclaw-talons/
├── LICENSE              # Apache 2.0 License
├── README.md            # This file
├── CONTRIBUTING.md      # Contribution guidelines
├── github/             # GitHub integration talon
├── docker/             # Docker management talon
├── search/             # Web search talon
└── ...
```

---

## Talon Manifest Reference

### Required Fields

| Field | Type | Description |
|-------|------|-------------|
| `name` | string | Unique talon name (lowercase, hyphenated) |
| `version` | string | Semantic version (e.g., 1.0.0) |
| `description` | string | One-line description |

### Optional Fields

| Field | Type | Description |
|-------|------|-------------|
| `author` | string | Author name |
| `license` | string | SPDX license identifier |
| `tags` | array | Tags for searching |
| `repository` | string | Source repository URL |
| `homepage` | string | Homepage URL |
| `runtime` | object | Runtime requirements |
| `permissions` | array | Required permissions |
| `environment` | array | Required environment variables |
| `commands` | array | Available commands |

### Example

```yaml
---
name: example
version: 1.0.0
description: An example talon
author: Your Name
license: MIT
tags: [example, demo]
runtime:
  kind: rust
  version: ">=1.70"
permissions:
  - shell
  - network
environment:
  - name: API_KEY
    required: true
    description: API key for service
commands:
  - name: greet
    description: Greets the user
    args:
      - name: name
        type: string
        required: true
---

# Example Talon

This is an example talon.
```

---

## Contributing

We welcome contributions from the community!

### How to Contribute

1. **Fork** this repository
2. **Create** a new talon directory
3. **Add** your TALON.md and supporting files
4. **Test** locally with `talon add`
5. **Submit** a pull request

### Guidelines

- Follow the TALON.md format
- Include clear documentation
- Test your talon before submitting
- Use Apache 2.0 license
- Add appropriate tags

See [CONTRIBUTING.md](CONTRIBUTING.md) for details.

---

## Security

When using talons:

- **Review** talon source code before installing
- **Audit** environment variables and permissions
- **Isolate** talons in production environments
- **Update** talons regularly for security patches

---

## Support

- **Issues**: Open an issue on GitHub
- **Discussions**: Use GitHub Discussions
- **Discord**: Join our community

---

## Roadmap

- [ ] Docker talon
- [ ] GitHub Actions talon
- [ ] Kubernetes talon
- [ ] Database talon
- [ ] AWS talon
- [ ] Azure talon

---

## License

Apache License 2.0 - see [LICENSE](LICENSE) file for details.

Copyright (c) 2026 FemtoClaw Community
