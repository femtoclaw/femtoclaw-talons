---
name: search
version: 1.0.0
description: Web search capabilities using multiple search providers
author: FemtoClaw Community
license: Apache-2.0
tags: [search, web, information, tools]
repository: https://github.com/femtoclaw/femtoclaw-skills
runtime:
  kind: rust
  version: ">=1.70"
permissions:
  - network
environment:
  - name: SEARCH_API_KEY
    required: false
    description: Optional API key for premium search
commands:
  - name: web
    description: Perform a web search
    args:
      - name: query
        type: string
        required: true
        description: Search query
      - name: limit
        type: integer
        required: false
        description: Number of results to return
        default: 10
  - name: wikipedia
    description: Search Wikipedia
    args:
      - name: query
        type: string
        required: true
        description: Search query
  - name: news
    description: Search for recent news
    args:
      - name: query
        type: string
        required: true
        description: Search query
      - name: limit
        type: integer
        required: false
        description: Number of results
        default: 5
---

# Search Skill

Provides web search capabilities to FemtoClaw using multiple search providers.

## Features

- **Web Search**: General web search using DuckDuckGo (no API key required)
- **Wikipedia Search**: Search Wikipedia articles
- **News Search**: Recent news articles

## Requirements

### Optional: API Keys

For enhanced search results, you can set:

- `SEARCH_API_KEY`: Optional API key for premium search providers

The skill works without API keys using free providers.

## Commands

### web

Perform a general web search:

```json
{"tool_call": {"tool": "search.web", "args": {"query": "FemtoClaw Rust agent", "limit": 5}}}
```

### wikipedia

Search Wikipedia:

```json
{"tool_call": {"tool": "search.wikipedia", "args": {"query": "artificial intelligence"}}}
```

### news

Search recent news:

```json
{"tool_call": {"tool": "search.news", "args": {"query": "Rust programming", "limit": 5}}}
```

## Output Format

Search results are returned as structured data:

```json
{
  "results": [
    {
      "title": "Result Title",
      "url": "https://example.com",
      "snippet": "Brief description..."
    }
  ]
}
```

## Privacy Notes

- Searches may be logged by search providers
- Use VPN for enhanced privacy if needed
- Review search provider privacy policies

## Rate Limits

- DuckDuckGo: ~30 requests/minute (no key)
- Wikipedia: ~200 requests/minute (no key)

## Use Cases

- Research and information gathering
- Finding documentation
- Staying current with news
- Troubleshooting and debugging
