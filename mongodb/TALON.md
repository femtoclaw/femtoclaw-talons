---
name: mongodb
version: 1.0.0
description: MongoDB document database operations
author: FemtoClaw Community
license: Apache-2.0
tags: [mongodb, database, nosql, document]
repository: https://github.com/femtoclaw/femtoclaw-talons
runtime:
  kind: shell
  version: ">=1.0"
permissions:
  - shell
  - database
environment:
  - name: MONGODB_URI
    required: true
    description: MongoDB connection URI
  - name: MONGODB_DATABASE
    required: false
    description: Default database name
commands:
  - name: find
    description: Find documents
    args:
      - name: collection
        type: string
        required: true
        description: Collection name
      - name: filter
        type: object
        required: false
        description: Query filter
      - name: limit
        type: integer
        required: false
        description: Result limit
        default: 10
  - name: insert
    description: Insert document
    args:
      - name: collection
        type: string
        required: true
        description: Collection name
      - name: document
        type: object
        required: true
        description: Document to insert
  - name: update
    description: Update documents
    args:
      - name: collection
        type: string
        required: true
        description: Collection name
      - name: filter
        type: object
        required: true
        description: Query filter
      - name: update
        type: object
        required: true
        description: Update operations
  - name: delete
    description: Delete documents
    args:
      - name: collection
        type: string
        required: true
        description: Collection name
      - name: filter
        type: object
        required: true
        description: Query filter
  - name: collections
    description: List collections
    args: []
---

# MongoDB Talon

MongoDB document database operations.

## Setup

```bash
export MONGODB_URI=mongodb://localhost:27017
export MONGODB_DATABASE=mydb
```

## Usage

### Find

```json
{"tool_call": {"tool": "mongodb.find", "args": {
  "collection": "users",
  "filter": {"active": true},
  "limit": 10
}}}
```

### Insert

```json
{"tool_call": {"tool": "mongodb.insert", "args": {
  "collection": "users",
  "document": {"name": "John", "email": "john@example.com"}
}}}
```

## Operations

- find - Query documents
- insert - Insert documents
- update - Update documents
- delete - Delete documents
- collections - List collections
