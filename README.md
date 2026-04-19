# 🐾 FemtoClaw Skills: The Ecosystem of Capabilities

[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://www.apache.org/licenses/LICENSE-2.0)

**FemtoClaw Skills** are high-level capability extensions that provide structured system integrations for the FemtoClaw runtime. While the core "Claws" (Shell, Net, FS) provide the fundamental execution substrate, Skills provide the specialized logic required to interact with cloud providers, communication platforms, databases, and enterprise software.

This repository serves as the official community hub for discovering, sharing, and developing new capabilities for the FemtoClaw ecosystem.

---

## 🏛️ What is a Skill?

A **Skill** is a self-contained capability module defined by a normative manifest (`SKILL.md`). It can be as simple as a configuration wrapper for a CLI tool or as complex as a compiled WebAssembly module with its own internal state.

### The Grasp & Control Model
Skills follow the same security philosophy as the core runtime:
1.  **Grasp**: They extend the "reach" of an agent into a new system (e.g., AWS, Slack).
2.  **Control**: They subject that interaction to strict JSON schema validation and rule-based policy enforcement.

---

## 🚀 Quick Installation & Usage

To manage skills, use the `skill` CLI provided by the **[femtoclaw-registry](../femtoclaw-registry)** crate.

```bash
# 1. Install the registry client
cargo install femtoclaw-registry

# 2. Add an official skill from the community hub
skill add github

# 3. Add a custom local skill during development
skill add ./path/to/my-skill
```

Once added, the new capabilities are automatically registered with your FemtoClaw agents, subject to your local authorization policies.

---

## 🧱 Available Skill Categories

The ecosystem is organized into functional categories to help developers find the right tools for their autonomous tasks.

### 1. Cloud & Infrastructure (IaC)
- **`aws` / `azure` / `gcp`**: Core cloud provider integrations for managing instances, buckets, and serverless functions.
- **`kubernetes`**: Pod inspection, deployment scaling, and log streaming.
- **`terraform`**: Execute `plan` and `apply` operations within an authorized sandbox.

### 2. Development & SCM
- **`git` / `github` / `gitlab`**: Perform commits, open PRs, and manage repository issues autonomously.
- **`terminal`**: Enhanced interactive shell features with built-in output sanitization.

### 3. Communication & Alerts
- **`slack` / `discord` / `telegram`**: Send notifications or participate in multi-agent chat channels.
- **`email`**: SMTP/IMAP bridge for legacy enterprise integration.

### 4. Persistence & Search
- **`postgres` / `mysql` / `redis`**: Direct structured data interaction.
- **`elasticsearch`**: Vector and full-text search capabilities for long-term memory retrieval.

---

## 📐 Creating a New Skill

Skills are designed to be "Drafting-First." You start by defining the interface in a `SKILL.md` manifest using YAML frontmatter.

```yaml
---
name: industrial-sensor-bridge
version: 1.0.0
description: Authorized access to factory floor telemetry.
author: Engineering Team
tags: [iot, factory, automation]
commands:
  - name: get_telemetry
    description: Fetch current sensor readings.
    args:
      - name: sensor_id
        type: string
        required: true
---
# Industrial Sensor Bridge
Provide your documentation here.
```

### Development Workflow:
1.  **Define Interface**: Create the `SKILL.md` manifest with strict argument schemas.
2.  **Implement Logic**: Write the underlying Rust, WASM, or shell scripts.
3.  **Local Test**: Use `skill add ./` to verify the integration in your local dev runtime.
4.  **Audit**: Ensure the skill produces structured events compatible with `femtoclaw-audit`.

---

## 🤝 Contribution & Governance

We welcome community submissions! To add your skill to the official hub:
1.  **Fork** this repository.
2.  **Create** a new directory for your skill.
3.  **Submit** a PR including the manifest, source, and a comprehensive `README.md` for the specific skill.

All official skills are subject to security review by the **FemtoClaw Engineering Authority**.

---

## 📄 License
The Skill ecosystem is open-source under the **Apache License 2.0**.

Copyright © 2026 FemtoClaw Project.
