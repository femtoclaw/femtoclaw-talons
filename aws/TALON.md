---
name: aws
version: 1.0.0
description: AWS operations - EC2, S3, Lambda, and more via AWS CLI
author: FemtoClaw Community
license: Apache-2.0
tags: [aws, cloud, devops, infrastructure]
repository: https://github.com/femtoclaw/femtoclaw-talons
runtime:
  kind: shell
  version: ">=1.0"
permissions:
  - shell
  - aws
environment:
  - name: AWS_ACCESS_KEY_ID
    required: true
    description: AWS access key ID
  - name: AWS_SECRET_ACCESS_KEY
    required: true
    description: AWS secret access key
  - name: AWS_REGION
    required: false
    description: AWS region
    default: us-east-1
commands:
  - name: s3_list
    description: List S3 buckets or objects
    args:
      - name: bucket
        type: string
        required: false
        description: S3 bucket name
      - name: prefix
        type: string
        required: false
        description: Object prefix
  - name: s3_copy
    description: Copy file to S3
    args:
      - name: source
        type: string
        required: true
        description: Source file path
      - name: destination
        type: string
        required: true
        description: S3 destination (s3://bucket/key)
  - name: ec2_list
    description: List EC2 instances
    args:
      - name: state
        type: string
        required: false
        description: Filter by state (running, stopped)
  - name: ec2_start
    description: Start an EC2 instance
    args:
      - name: instance_id
        type: string
        required: true
        description: EC2 instance ID
  - name: ec2_stop
    description: Stop an EC2 instance
    args:
      - name: instance_id
        type: string
        required: true
        description: EC2 instance ID
  - name: lambda_list
    description: List Lambda functions
    args: []
  - name: lambda_invoke
    description: Invoke a Lambda function
    args:
      - name: function_name
        type: string
        required: true
        description: Lambda function name
      - name: payload
        type: string
        required: false
        description: JSON payload
---

# AWS Talon

Provides AWS cloud operations for FemtoClaw via AWS CLI.

## Features

- **S3 Operations**: List, copy, sync files
- **EC2 Management**: Start, stop, list instances
- **Lambda Functions**: List and invoke functions
- **General AWS**: Full AWS CLI access

## Requirements

### AWS CLI

Install AWS CLI:

```bash
# macOS
brew install awscli

# Linux
curl "https://awscli.amazonaws.com/awscli-exe-linux-x86_64.zip" -o "awscliv2.zip"
unzip awscliv2.zip
sudo ./aws/install
```

### Configuration

Set AWS credentials:

```bash
export AWS_ACCESS_KEY_ID=AKIAIOSFODNN7EXAMPLE
export AWS_SECRET_ACCESS_KEY=wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY
export AWS_REGION=us-east-1
```

Or configure using:

```bash
aws configure
```

## Commands

### s3_list

List S3 buckets:

```bash
aws s3 ls
```

List objects in bucket:

```bash
aws s3 ls s3://my-bucket/
```

### ec2_list

List EC2 instances:

```bash
aws ec2 describe-instances --filters "Name=instance-state-code,Values=16"
```

### lambda_invoke

Invoke Lambda:

```json
{"tool_call": {"tool": "aws.lambda_invoke", "args": {
  "function_name": "my-function",
  "payload": "{\"key\": \"value\"}"
}}}
```

## Use Cases

- Cloud infrastructure management
- Serverless function invocation
- S3 file operations
- EC2 instance management
- Deployment automation

## Security

- Use IAM roles when possible
- Apply least-privilege permissions
- Enable MFA for sensitive operations
- Rotate access keys regularly
- Never commit credentials
- Use AWS Secrets Manager for production

## Supported Services

- EC2 (compute)
- S3 (storage)
- Lambda (serverless)
- IAM (access)
- CloudWatch (monitoring)
- (Extensible for more)
