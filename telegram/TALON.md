---
name: telegram
version: 1.0.0
description: Telegram integration for sending messages
author: FemtoClaw Community
license: Apache-2.0
tags: [telegram, notifications, chat, integration]
repository: https://github.com/femtoclaw/femtoclaw-talons
runtime:
  kind: shell
  version: ">=1.0"
permissions:
  - shell
  - network
environment:
  - name: TELEGRAM_BOT_TOKEN
    required: true
    description: Telegram bot token
  - name: TELEGRAM_CHAT_ID
    required: true
    description: Telegram chat ID
commands:
  - name: send
    description: Send message to Telegram
    args:
      - name: text
        type: string
        required: true
        description: Message text
      - name: parse_mode
        type: string
        required: false
        description: Parse mode (Markdown/HTML)
        default: Markdown
---

# Telegram Talon

Send Telegram messages via bot API.

## Setup

1. Create bot via @BotFather on Telegram
2. Get bot token
3. Get chat ID (use @userinfobot)

Set:

```bash
export TELEGRAM_BOT_TOKEN=123456:ABC-DEF1234ghIkl-zyx57W2v1u123ew11
export TELEGRAM_CHAT_ID=123456789
```

## Usage

```json
{"tool_call": {"tool": "telegram.send", "args": {
  "text": "Hello from FemtoClaw!"
}}}
```
