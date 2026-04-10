---
name: azure-cloud
version: 1.0.0
description: Microsoft Azure cloud operations - VMs, Container Instances, Functions
author: FemtoClaw Community
license: Apache-2.0
tags: [azure, cloud, microsoft, vm, aks, container-instances]
repository: https://github.com/femtoclaw/femtoclaw-talons
---

# Azure Cloud Talon

Provides Microsoft Azure cloud operations for VMs, containers, and serverless.

## What This Talon Does

Manages Azure resources including virtual machines, container instances, and Azure functions.

## Commands

### list_vms
Lists Azure virtual machines.

**Arguments:**
- `resource_group` (string, required): Resource group name

### create_vm
Creates a new virtual machine.

**Arguments:**
- `name` (string, required): VM name
- `resource_group` (string, required): Resource group
- `location` (string, required): Azure region
- `size` (string, optional): VM size

### start_vm
Starts a virtual machine.

**Arguments:**
- `name` (string, required): VM name
- `resource_group` (string, required): Resource group

### stop_vm
Stops a virtual machine.

**Arguments:**
- `name` (string, required): VM name
- `resource_group` (string, required): Resource group

### list_containers
Lists Azure Container Instances.

**Arguments:**
- `resource_group` (string, required): Resource group name

### deploy_function
Deploys an Azure Function.

**Arguments:**
- `name` (string, required): Function name
- `resource_group` (string, required): Resource group
- `runtime` (string, required): Runtime (nodejs, python, dotnet)

### list_storage
Lists Azure Storage accounts.

**Arguments:**
- `resource_group` (string, optional): Resource group filter

## Usage

```json
{"tool_call": {"tool": "azure-cloud.list_vms", "args": {"resource_group": "my-rg"}}}
```

## Notes

- Requires Azure CLI (az) and logged in credentials
- Supports service principal authentication
