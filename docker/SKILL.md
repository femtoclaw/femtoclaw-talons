---
name: docker
version: 1.0.0
description: Docker container management - build, run, and manage containers
author: FemtoClaw Community
license: Apache-2.0
tags: [docker, devops, containers, cloud]
repository: https://github.com/femtoclaw/femtoclaw-skills
runtime:
  kind: shell
  version: ">=1.0"
permissions:
  - shell
  - docker
commands:
  - name: list_containers
    description: List running containers
    args:
      - name: all
        type: boolean
        required: false
        description: Show all containers
        default: false
  - name: list_images
    description: List Docker images
    args: []
  - name: run
    description: Run a Docker container
    args:
      - name: image
        type: string
        required: true
        description: Docker image name
      - name: name
        type: string
        required: false
        description: Container name
      - name: detach
        type: boolean
        required: false
        description: Run in detached mode
        default: true
  - name: stop
    description: Stop a running container
    args:
      - name: container
        type: string
        required: true
        description: Container name or ID
  - name: logs
    description: Fetch container logs
    args:
      - name: container
        type: string
        required: true
        description: Container name or ID
      - name: tail
        type: integer
        required: false
        description: Number of lines to show
        default: 100
---

# Docker Skill

Provides Docker container management capabilities for FemtoClaw.

## Features

- **Container Management**: Start, stop, list containers
- **Image Management**: List and manage Docker images
- **Log Viewing**: Fetch and display container logs
- **Interactive Runs**: Run containers interactively

## Requirements

### Docker

This skill requires Docker to be installed:

```bash
# Verify installation
docker --version
```

### Permissions

- Docker socket access (`/var/run/docker.sock` on Linux)
- Appropriate user permissions

## Commands

### list_containers

List running containers:

```bash
docker ps
```

### run

Run a new container:

```bash
docker run -d nginx
```

### stop

Stop a running container:

```bash
docker stop my-container
```

### logs

View container logs:

```bash
docker logs --tail 100 my-container
```

## Security Considerations

- Limit container capabilities
- Use read-only root filesystems when possible
- Avoid running containers as root
- Scan images for vulnerabilities

## Use Cases

- CI/CD pipeline automation
- Development environment management
- Container orchestration
- Deployment automation
