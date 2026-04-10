#!/bin/bash
# AWS Talon - List EC2 instances

REGION="${1:-us-east-1}"

aws ec2 describe-instances --region "$REGION" --query 'Reservations[*].Instances[*].[InstanceId,State.Name,InstanceType]' --output table
