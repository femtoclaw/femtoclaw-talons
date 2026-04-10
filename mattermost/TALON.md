---
name: mattermost
version: 1.0.0
description: Mattermost team chat integration for messaging and notifications
author: FemtoClaw Community
license: Apache-2.0
tags: [mattermost, chat, messaging, team, notifications]
repository: https://github.com/femtoclaw/femtoclaw-talons
---

# Mattermost Talon

Integrates with Mattermost for team messaging and notifications.

## What This Talon Does

Sends messages, manages channels, and interacts with Mattermost teams.

## Commands

### send_message
Sends a message to a channel or user.

**Arguments:**
- `channel` (string, required): Channel name or ID
- `message` (string, required): Message content
- `root_id` (string, optional): Root message ID for threads

### list_channels
Lists available channels.

**Arguments:**
- `team` (string, optional): Team name

### create_channel
Creates a new channel.

**Arguments:**
- `name` (string, required): Channel name
- `team` (string, required): Team name
- `display_name` (string, required): Display name
- `type` (string, optional): Channel type (O for public, P for private)

### list_users
Lists users in the team.

**Arguments:**
- `team` (string, optional): Team name

### send_dm
Sends a direct message to a user.

**Arguments:**
- `user` (string, required): Username
- `message` (string, required): Message content

### upload_file
Uploads a file to a channel.

**Arguments:**
- `channel` (string, required): Channel name
- `file_path` (string, required): Path to file

## Usage

```json
{"tool_call": {"tool": "mattermost.send_message", "args": {"channel": "dev-team", "message": "Build complete!"}}}
```

## Notes

- Requires MATTERMOST_URL, MATTERMOST_USER, and MATTERMOST_TOKEN environment variables
- Supports webhook-based integrations
