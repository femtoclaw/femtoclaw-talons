---
name: discord
version: 1.0.0
description: Discord integration for sending messages and notifications
author: FemtoClaw Community
license: Apache-2.0
tags: [discord, notifications, chat, integration]
repository: https://github.com/femtoclaw/femtoclaw-skills
runtime:
  kind: shell
  version: ">=1.0"
permissions:
  - shell
  - network
environment:
  - name: DISCORD_WEBHOOK_URL
    required: true
    description: Discord webhook URL
commands:
  - name: send
    description: Send message to channel
    args:
      - name: content
        type: string
        required: true
        description: Message content
      - name: username
        type: string
        required: false
        description: Override username
        default: FemtoClaw
---

# Discord Skill

Send messages to Discord via webhooks.

## Setup

1. Go to Server Settings > Integrations > Webhooks
2. Create a new webhook
3. Copy the URL

Set:

```bash
export DISCORD_WEBHOOK_URL=https://discord.com/api/webhooks/...
```

## Usage

```json
{"tool_call": {"tool": "discord.send", "args": {
  "content": "Build completed!"
}}}
```
