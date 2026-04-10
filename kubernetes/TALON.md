---
name: kubernetes
version: 1.0.0
description: Kubernetes management - pods, services, deployments, and more
author: FemtoClaw Community
license: Apache-2.0
tags: [kubernetes, k8s, devops, containers, cloud]
repository: https://github.com/femtoclaw/femtoclaw-talons
runtime:
  kind: shell
  version: ">=1.0"
permissions:
  - shell
  - kubernetes
environment:
  - name: KUBECONFIG
    required: false
    description: Path to kubeconfig file
commands:
  - name: pods
    description: List pods in a namespace
    args:
      - name: namespace
        type: string
        required: false
        description: Kubernetes namespace
        default: default
  - name: pods_logs
    description: Get pod logs
    args:
      - name: pod
        type: string
        required: true
        description: Pod name
      - name: namespace
        type: string
        required: false
        description: Kubernetes namespace
        default: default
      - name: tail
        type: integer
        required: false
        description: Lines to show
        default: 100
  - name: deployments
    description: List deployments
    args:
      - name: namespace
        type: string
        required: false
        description: Kubernetes namespace
        default: default
  - name: scale_deployment
    description: Scale a deployment
    args:
      - name: deployment
        type: string
        required: true
        description: Deployment name
      - name: replicas
        type: integer
        required: true
        description: Number of replicas
      - name: namespace
        type: string
        required: false
        description: Kubernetes namespace
        default: default
  - name: services
    description: List services
    args:
      - name: namespace
        type: string
        required: false
        description: Kubernetes namespace
        default: default
  - name: apply
    description: Apply a Kubernetes manifest
    args:
      - name: manifest
        type: string
        required: true
        description: Path to manifest file or YAML content
  - name: delete
    description: Delete a resource
    args:
      - name: resource_type
        type: string
        required: true
        description: Resource type (pod, service, etc.)
      - name: name
        type: string
        required: true
        description: Resource name
      - name: namespace
        type: string
        required: false
        description: Kubernetes namespace
---

# Kubernetes Talon

Provides Kubernetes cluster management for FemtoClaw.

## Features

- **Pod Management**: List, logs, exec into pods
- **Deployment Operations**: Scale, update, rollback
- **Service Discovery**: List and manage services
- **Manifest Application**: Apply YAML manifests

## Requirements

### kubectl

Install kubectl:

```bash
# macOS
brew install kubectl

# Linux
curl -LO "https://dl.k8s.io/release/$(curl -L -s https://dl.k8s.io/release/stable.txt)/bin/linux/amd64/kubectl"
```

### Configuration

Set up kubeconfig:

```bash
export KUBECONFIG=/path/to/kubeconfig
```

Or use:

```bash
kubectl config use-context context-name
```

## Commands

### pods

List pods:

```bash
kubectl get pods -n default
```

### pods_logs

Get pod logs:

```bash
kubectl logs my-pod -n default --tail=100
```

### deployments

List deployments:

```bash
kubectl get deployments -n default
```

### scale_deployment

Scale a deployment:

```json
{"tool_call": {"tool": "kubernetes.scale_deployment", "args": {
  "deployment": "my-app",
  "replicas": 3,
  "namespace": "production"
}}}
```

### apply

Apply a manifest:

```bash
kubectl apply -f deployment.yaml
```

## Use Cases

- Deployment automation
- Pod log monitoring
- Scaling operations
- Service management
- CI/CD pipelines

## Security

- Use RBAC properly
- Avoid running as root
- Scan for vulnerabilities
- Use network policies
- Enable audit logging
- Rotate credentials

## Common Operations

### Restart a deployment

```bash
kubectl rollout restart deployment/my-app -n default
```

### Get resource usage

```bash
kubectl top pods -n default
```

### Port-forward to pod

```bash
kubectl port-forward pod/my-pod 8080:80
```
