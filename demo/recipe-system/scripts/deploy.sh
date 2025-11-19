#!/bin/bash
set -e

echo "Deploying Recipe Management System to WasmCloud..."

# Check if wasmCloud is running
if ! wash get hosts > /dev/null 2>&1; then
    echo "WasmCloud is not running. Starting wasmCloud..."
    wash up -d
    echo "Waiting for wasmCloud to start..."
    sleep 5
fi

# Deploy using wadm
echo "Deploying application..."
wadm app deploy wadm.yaml

# Wait for deployment
echo "Waiting for deployment to complete..."
sleep 5

# Check status
echo "Checking deployment status..."
wadm app get recipe-system

echo ""
echo "Deployment complete!"
echo "API available at: http://localhost:8080"
echo ""
echo "Test with:"
echo "  curl http://localhost:8080/health"
echo "  curl http://localhost:8080/api/recipes"
