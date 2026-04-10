#!/bin/bash
# MySQL Talon - List databases

mysql -h "$MYSQL_HOST" -u "$MYSQL_USER" -p"$MYSQL_PASSWORD" -e "SHOW DATABASES;"
