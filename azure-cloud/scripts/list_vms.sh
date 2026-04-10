#!/bin/bash
# Azure Cloud Talon - List VMs

RESOURCE_GROUP="${1:-}"

if [ -z "$RESOURCE_GROUP" ]; then
    echo "Usage: list_vms.sh <resource-group>"
    exit 1
fi

az vm list -g "$RESOURCE_GROUP" -o table
