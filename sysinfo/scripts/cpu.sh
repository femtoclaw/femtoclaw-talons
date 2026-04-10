#!/bin/bash
# Sysinfo Talon - Show CPU info

echo "CPU Info:"
if command -v lscpu &> /dev/null; then
    lscpu
else
    cat /proc/cpuinfo
fi
