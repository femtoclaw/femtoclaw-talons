---
name: webhook
version: 1.0.0
description: Generic webhook integration for sending HTTP POST requests
author: FemtoClaw Community
license: Apache-2.0
tags: [webhook, http, integration, automation, notifications]
repository: https://github.com/femtoclaw/femtoclaw-skills
---

# Webhook Skill

Sends HTTP POST requests to generic webhook endpoints.

## What This Skill Does

Provides flexible webhook integration for triggering external services and APIs.

## Commands

### send
Sends a generic HTTP POST request.

**Arguments:**
- `url` (string, required): Webhook URL
- `payload` (object, required): JSON payload
- `headers` (object, optional): Custom headers
- `method` (string, optional): HTTP method (POST, PUT, PATCH)

### send_form
Sends form-encoded data.

**Arguments:**
- `url` (string, required): Webhook URL
- `data` (object, required): Form data

### send_file
Sends a file via multipart POST.

**Arguments:**
- `url` (string, required): Webhook URL
- `file_path` (string, required): Path to file
- `field_name` (string, optional): Form field name

### test
Tests a webhook endpoint.

**Arguments:**
- `url` (string, required): Webhook URL

### list_webhooks
Lists configured webhook endpoints.

## Usage

```json
{"tool_call": {"tool": "webhook.send", "args": {"url": "https://example.com/hook", "payload": {"event": "build", "status": "success"}}}}
```

## Notes

- Supports custom headers for authentication
- Can retry failed requests
- Webhook URLs stored securely in config
