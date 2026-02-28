---
name: teams
version: 1.0.0
description: Microsoft Teams integration for sending notifications
author: FemtoClaw Community
license: Apache-2.0
tags: [teams, microsoft, notifications, integration]
repository: https://github.com/femtoclaw/femtoclaw-talons
runtime:
  kind: shell
  version: ">=1.0"
permissions:
  - shell
  - network
environment:
  - name: TEAMS_WEBHOOK_URL
    required: true
    description: Teams incoming webhook URL
commands:
  - name: send
    description: Send message to Teams channel
    args:
      - name: title
        type: string
        required: true
        description: Message title
      - name: text
        type: string
        required: true
        description: Message text
      - name: theme_color
        type: string
        required: false
        description: Accent color (hex)
        default: "0078D4"
---

# Teams Talon

Send Microsoft Teams notifications via incoming webhooks.

## Setup

1. Go to Channel > Connectors > Incoming Webhook
2. Create webhook
3. Copy URL

Set:

```bash
export TEAMS_WEBHOOK_URL=https://outlook.office.com/webhook/...
```

## Usage

```json
{"tool_call": {"tool": "teams.send", "args": {
  "title": "Deployment",
  "text": "Deployment completed successfully",
  "theme_color": "00FF00"
}}}
```
