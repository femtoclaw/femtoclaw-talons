#!/bin/bash
# GCP Talon - List compute instances

PROJECT="${1:-}"
ZONE="${2:-us-central1-a}"

if [ -z "$PROJECT" ]; then
    echo "Usage: list_instances.sh <project> [zone]"
    exit 1
fi

gcloud compute instances list --project="$PROJECT" --zone="$ZONE"
