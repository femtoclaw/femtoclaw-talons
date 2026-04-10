#!/bin/bash
# Terraform Talon - Plan changes

PATH="${1:-.}"

cd "$PATH" && terraform plan
