---
name: matrix
version: 1.0.0
description: Matrix protocol chat integration for decentralized messaging
author: FemtoClaw Community
license: Apache-2.0
tags: [matrix, chat, messaging, decentralized, element]
repository: https://github.com/femtoclaw/femtoclaw-skills
---

# Matrix Skill

Integrates with Matrix protocol for decentralized team messaging.

## What This Skill Does

Sends messages, manages rooms, and interacts with Matrix networks.

## Commands

### send_message
Sends a message to a Matrix room.

**Arguments:**
- `room` (string, required): Room ID or alias
- `message` (string, required): Message content
- `html` (string, optional): HTML formatted message

### create_room
Creates a new Matrix room.

**Arguments:**
- `name` (string, required): Room name
- `topic` (string, optional): Room topic
- `is_private` (boolean, optional): Make room private

### join_room
Joins a Matrix room.

**Arguments:**
- `room` (string, required): Room ID or alias

### leave_room
Leaves a Matrix room.

**Arguments:**
- `room` (string, required): Room ID or alias

### list_rooms
Lists joined rooms.

### send_dm
Sends a direct message to a user.

**Arguments:**
- `user` (string, required): User ID
- `message` (string, required): Message content

### get_state
Gets room state.

**Arguments:**
- `room` (string, required): Room ID

## Usage

```json
{"tool_call": {"tool": "matrix.send_message", "args": {"room": "#femtoclaw:matrix.org", "message": "Hello from FemtoClaw!"}}}
```

## Notes

- Requires MATRIX_HOMESERVER, MATRIX_USER, and MATRIX_TOKEN environment variables
- Works with any Matrix-compatible server (Matrix.org, Element, etc.)
