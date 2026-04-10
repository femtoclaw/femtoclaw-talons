#!/bin/bash
# GitLab Talon - List projects

curl -s "https://gitlab.com/api/v4/projects" -H "PRIVATE-TOKEN: $GITLAB_TOKEN" | jq '.[] | {name: .name, path: .path_with_namespace}'
