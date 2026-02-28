#!/bin/bash
# Elasticsearch Talon - List indices

URL="${ELASTICSEARCH_URL:-http://localhost:9200}"

curl -s "$URL/_cat/indices"
