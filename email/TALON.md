---
name: email
version: 1.0.0
description: Email sending and management via SMTP
author: FemtoClaw Community
license: Apache-2.0
tags: [email, notifications, smtp, communication]
repository: https://github.com/femtoclaw/femtoclaw-talons
runtime:
  kind: shell
  version: ">=1.0"
permissions:
  - shell
  - network
environment:
  - name: SMTP_HOST
    required: true
    description: SMTP server hostname
  - name: SMTP_PORT
    required: false
    description: SMTP port
    default: "587"
  - name: SMTP_USER
    required: true
    description: SMTP username
  - name: SMTP_PASSWORD
    required: true
    description: SMTP password
  - name: SMTP_FROM
    required: true
    description: From email address
commands:
  - name: send
    description: Send an email
    args:
      - name: to
        type: string
        required: true
        description: Recipient email(s), comma-separated
      - name: subject
        type: string
        required: true
        description: Email subject
      - name: body
        type: string
        required: true
        description: Email body
      - name: cc
        type: string
        required: false
        description: CC recipients
      - name: bcc
        type: string
        required: false
        description: BCC recipients
      - name: html
        type: boolean
        required: false
        description: Send as HTML
        default: false
---

# Email Talon

Provides email sending capabilities for FemtoClaw via SMTP.

## Features

- **Send Emails**: Plain text and HTML
- **Multiple Recipients**: TO, CC, BCC support
- **SMTP Auth**: Secure authentication
- **Templates**: Support for email templates

## Requirements

### SMTP Configuration

Set environment variables:

```bash
export SMTP_HOST=smtp.gmail.com
export SMTP_PORT=587
export SMTP_USER=your@email.com
export SMTP_PASSWORD=your-app-password
export SMTP_FROM=Your Name <your@email.com>
```

For Gmail, use an App Password:
https://support.google.com/accounts/answer/185833

## Commands

### send

Send a simple email:

```json
{"tool_call": {"tool": "email.send", "args": {
  "to": "recipient@example.com",
  "subject": "Hello",
  "body": "This is the email body"
}}}
```

Send HTML email:

```json
{"tool_call": {"tool": "email.send", "args": {
  "to": "recipient@example.com",
  "subject": "HTML Email",
  "body": "<h1>Hello!</h1><p>This is HTML.</p>",
  "html": true
}}}
```

## Use Cases

- Transactional emails
- Notifications
- Alerts and monitoring
- Report delivery
- User onboarding

## Security

- Use App Passwords (not login passwords)
- Enable 2FA on email accounts
- Store credentials securely
- Use TLS/SSL (port 587 or 465)
- Limit email sending rate

## Email Providers

### Gmail
- Host: smtp.gmail.com
- Port: 587 (TLS) or 465 (SSL)
- Use App Password

### Outlook
- Host: smtp.office365.com
- Port: 587 (TLS)

### Amazon SES
- Host: email-smtp.us-east-1.amazonaws.com
- Port: 587 (TLS)
