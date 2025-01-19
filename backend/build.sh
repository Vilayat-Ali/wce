#!/bin/bash

# Build entrypoint app
# echo "Building entrypoint app..."
# docker-compose build entrypoint

# Build player service
echo "Building player service..."
docker-compose build playerservice

# Build game service
# echo "Building game service..."
# docker-compose build gameservice

# # Build editor service
# echo "Building editor service..."
# docker-compose build editorservice

# # Build database and redis services (optional, only if you need to build them too)
# echo "Building database service..."
# docker-compose build db

# echo "Building redis service..."
# docker-compose build redis

# echo "All services have been built."
