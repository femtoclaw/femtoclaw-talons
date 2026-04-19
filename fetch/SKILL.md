---
name: fetch
version: 1.0.0
description: URL content fetching with headers and authentication
author: FemtoClaw Community
license: Apache-2.0
tags: [fetch, http, network, utilities]
repository: https://github.com/femtoclaw/femtoclaw-skills
runtime:
  kind: shell
  version: ">=1.0"
permissions:
  - shell
  - network
commands:
  - name: get
    description: GET request to URL
    args:
      - name: url
        type: string
        required: true
        description: Target URL
      - name: headers
        type: object
        required: false
        description: Custom headers
  - name: post
    description: POST request to URL
    args:
      - name: url
        type: string
        required: true
        description: Target URL
      - name: body
        type: string
        required: false
        description: Request body
      - name: content_type
        type: string
        required: false
        description: Content type
        default: application/json
  - name: json
    description: Fetch and parse JSON
    args:
      - name: url
        type: string
        required: true
        description: JSON API URL
---

# Fetch Skill

HTTP request capabilities for FemtoClaw.

## Features

- GET/POST requests
- Custom headers
- JSON parsing
- Response handling

## Usage

### GET

```json
{"tool_call": {"tool": "fetch.get", "args": {
  "url": "https://api.example.com/data"
}}}
```

### POST

```json
{"tool_call": {"tool": "fetch.post", "args": {
  "url": "https://api.example.com/submit",
  "body": "{\"key\": \"value\"}"
}}}
```

### JSON

```json
{"tool_call": {"tool": "fetch.json", "args": {
  "url": "https://jsonplaceholder.typicode.com/posts/1"
}}}
```
