---
name: gcp
version: 1.0.0
description: Google Cloud Platform operations - Compute, Storage, Functions, and more
author: FemtoClaw Community
license: Apache-2.0
tags: [gcp, google-cloud, cloud, compute, gke, cloud-run]
repository: https://github.com/femtoclaw/femtoclaw-talons
---

# GCP Talon

Provides Google Cloud Platform operations for Compute Engine, GKE, Cloud Run, and more.

## What This Talon Does

Manages GCP resources including compute instances, containers, and cloud functions.

## Commands

### list_instances
Lists Compute Engine instances.

**Arguments:**
- `project` (string, required): GCP project ID
- `zone` (string, optional): Zone filter

### create_instance
Creates a new Compute Engine instance.

**Arguments:**
- `name` (string, required): Instance name
- `project` (string, required): GCP project ID
- `zone` (string, required): Zone
- `machine_type` (string, optional): Machine type
- `image` (string, optional): Disk image

### start_instance
Starts a Compute Engine instance.

**Arguments:**
- `name` (string, required): Instance name
- `project` (string, required): GCP project ID
- `zone` (string, required): Zone

### stop_instance
Stops a Compute Engine instance.

**Arguments:**
- `name` (string, required): Instance name
- `project` (string, required): GCP project ID
- `zone` (string, required): Zone

### list_containers
Lists GKE clusters and containers.

**Arguments:**
- `project` (string, required): GCP project ID

### deploy_function
Deploys a Cloud Function.

**Arguments:**
- `name` (string, required): Function name
- `runtime` (string, required): Runtime (nodejs, python, go)
- `source` (string, required): Source code path

## Usage

```json
{"tool_call": {"tool": "gcp.list_instances", "args": {"project": "my-project"}}}
```

## Notes

- Requires gcloud CLI and Application Default Credentials
- Supports service account authentication
