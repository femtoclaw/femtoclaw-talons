#!/bin/bash
# Cron Talon - List jobs

crontab -l 2>/dev/null || echo "No crontab"
