---
name: elasticsearch
version: 1.0.0
description: Elasticsearch operations - search, indexing, aggregations
author: FemtoClaw Community
license: Apache-2.0
tags: [elasticsearch, search, elastic, indexing, analytics]
repository: https://github.com/femtoclaw/femtoclaw-talons
---

# Elasticsearch Talon

Provides Elasticsearch operations for search and analytics.

## What This Talon Does

Manages Elasticsearch indices, documents, and search queries.

## Commands

### search
Executes a search query.

**Arguments:**
- `index` (string, required): Index name
- `query` (object, required): Search query
- `size` (number, optional): Results size
- `from` (number, optional): Results offset

### index
Indexes a document.

**Arguments:**
- `index` (string, required): Index name
- `id` (string, optional): Document ID
- `document` (object, required): Document to index

### bulk_index
Bulk indexes multiple documents.

**Arguments:**
- `index` (string, required): Index name
- `documents` (array, required): Array of documents

### create_index
Creates a new index.

**Arguments:**
- `index` (string, required): Index name
- `settings` (object, optional): Index settings
- `mappings` (object, optional): Index mappings

### delete_index
Deletes an index.

**Arguments:**
- `index` (string, required): Index name

### list_indices
Lists all indices.

### aggregations
Runs an aggregation query.

**Arguments:**
- `index` (string, required): Index name
- `aggs` (object, required): Aggregation definition
- `size` (number, optional): Results size

### delete
Deletes a document.

**Arguments:**
- `index` (string, required): Index name
- `id` (string, required): Document ID

## Usage

```json
{"tool_call": {"tool": "elasticsearch.search", "args": {"index": "logs", "query": {"match": {"message": "error"}}}}
```

## Notes

- Requires ELASTICSEARCH_URL and optional auth credentials
- Supports Elasticsearch 8.x
- Handles index templates and aliases
