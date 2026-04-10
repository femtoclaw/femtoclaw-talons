#!/bin/bash
# MongoDB Talon - List collections

mongosh "$MONGO_URI" --eval "db.getCollectionNames()"
