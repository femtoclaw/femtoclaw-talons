---
name: slack
version: 1.0.0
description: Slack integration for sending messages and notifications
author: FemtoClaw Community
license: Apache-2.0
tags: [slack, notifications, chat, integration]
repository: https://github.com/femtoclaw/femtoclaw-skills
runtime:
  kind: shell
  version: ">=1.0"
permissions:
  - shell
  - network
environment:
  - name: SLACK_TOKEN
    required: true
    description: Slack bot user OAuth token
commands:
  - name: send_message
    description: Send a message to a channel
    args:
      - name: channel
        type: string
        required: true
        description: Channel name (e.g., #general)
      - name: text
        type: string
        required: true
        description: Message text
      - name: thread
        type: string
        required: false
        description: Thread timestamp to reply to
  - name: send_dm
    description: Send a direct message
    args:
      - name: user
        type: string
        required: true
        description: User ID or email
      - name: text
        type: string
        required: true
        description: Message text
  - name: list_channels
    description: List Slack channels
    args: []
---

# Slack Skill

Provides Slack integration for sending messages and notifications.

## Features

- **Channel Messages**: Send messages to channels
- **Direct Messages**: Send DMs to users
- **Threading**: Reply to threads
- **Channel Listing**: List available channels

## Requirements

### Slack Bot Token

1. Create a Slack App at https://api.slack.com/apps
2. Add `chat:write` and `channels:read` scopes
3. Install to workspace
4. Set the token:

```bash
export SLACK_TOKEN=xoxb-your-token-here
```

### Bot Permissions

Required scopes:
- `chat:write` - Send messages
- `channels:read` - List channels
- `im:write` - Send DMs
- `users:read` - Read user info

## Commands

### send_message

Send message to a channel:

```json
{"tool_call": {"tool": "slack.send_message", "args": {"channel": "#general", "text": "Hello team!"}}}
```

### send_dm

Send direct message:

```json
{"tool_call": {"tool": "slack.send_dm", "args": {"user": "user@example.com", "text": "Hey!"}}}
```

### list_channels

List available channels:

```bash
curl -H "Authorization: Bearer $SLACK_TOKEN" \
  https://slack.com/api/conversations.list
```

## Use Cases

- Build notifications
- Deployment alerts
- Error reporting
- Team communication
- CI/CD status updates

## Security

- Store tokens securely
- Use least-privilege scopes
- Rotate tokens regularly
- Never commit tokens to source
