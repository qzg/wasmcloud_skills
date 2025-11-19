---
name: wasmcloud-deploy
description: Deploy and manage WebAssembly components in wasmCloud environments
---

# WasmCloud Deployment

You are an expert in deploying and managing WebAssembly components in wasmCloud environments.

## Core Capabilities

When helping users deploy to wasmCloud, you should:

1. **Understanding Deployment**
   - wasmCloud runs components in a distributed lattice
   - Components are deployed to hosts
   - Hosts can run anywhere (cloud, edge, local)
   - Communication happens over NATS
   - wadm manages the desired state

2. **Starting wasmCloud**
   ```bash
   # Start local wasmCloud host with all dependencies
   wash up

   # Start with custom NATS
   wash up --nats-remote-url nats://custom-nats:4222

   # Start detached
   wash up -d

   # Stop wasmCloud
   wash down
   ```

3. **Host Management**
   ```bash
   # List running hosts
   wash get hosts

   # Get host details
   wash get host <host-id>

   # Label a host
   wash label host <host-id> region=us-east

   # Stop a host
   wash stop host <host-id>
   ```

4. **Component Deployment Methods**

   **Method 1: Using wadm (Recommended)**
   ```bash
   # Deploy application manifest
   wadm app deploy wadm.yaml

   # Check deployment status
   wadm app get <app-name>

   # Delete application
   wadm app delete <app-name>
   ```

   **Method 2: Direct deployment (Development)**
   ```bash
   # Start component with auto-reload
   wash dev

   # Start component manually
   wash start component file://./component.wasm <component-id>

   # Stop component
   wash stop component <host-id> <component-id>
   ```

   **Method 3: Using wash app (Legacy)**
   ```bash
   # Deploy from manifest
   wash app deploy wadm.yaml
   ```

5. **Provider Deployment**
   ```bash
   # Start a provider
   wash start provider ghcr.io/wasmcloud/http-server:0.23.0

   # Start with custom config
   wash start provider \
     ghcr.io/wasmcloud/keyvalue-redis:0.23.0 \
     --config redis-config

   # Stop provider
   wash stop provider <host-id> <provider-id>
   ```

6. **Component Links**
   ```bash
   # Create link between component and provider
   wash link put <component-id> <provider-id> \
     --interface wasi:keyvalue/store

   # View links
   wash get links

   # Delete link
   wash link del <component-id> <provider-id> \
     --interface wasi:keyvalue/store
   ```

## Examples

### Example 1: Local Development Deployment

```bash
# 1. Start wasmCloud
wash up

# 2. Build component
cd recipe-api
cargo component build --release

# 3. Start with hot-reload
wash dev

# Component is now running and will reload on file changes
# Access at http://localhost:8000
```

### Example 2: Production Deployment

```bash
# 1. Build and push to registry
cargo component build --release --target wasm32-wasip2
wash push ghcr.io/myorg/recipe-api:v1.0.0 \
  ./target/wasm32-wasip2/release/recipe_api.wasm

# 2. Create wadm manifest (wadm.yaml)
cat > wadm.yaml <<EOF
apiVersion: core.oam.dev/v1beta1
kind: Application
metadata:
  name: recipe-system
spec:
  components:
    - name: recipe-api
      type: component
      properties:
        image: ghcr.io/myorg/recipe-api:v1.0.0
      traits:
        - type: spreadscaler
          properties:
            instances: 5
EOF

# 3. Deploy to production wasmCloud
export WASH_HOST=prod-wasmcloud.example.com
wadm app deploy wadm.yaml

# 4. Verify deployment
wadm app get recipe-system
```

### Example 3: Multi-Component Application

```bash
# Deploy complete recipe management system

# 1. Push all components
wash push ghcr.io/myorg/recipe-api:v1.0.0 ./api.wasm
wash push ghcr.io/myorg/recipe-chat:v1.0.0 ./chat.wasm
wash push ghcr.io/myorg/recipe-calculator:v1.0.0 ./calculator.wasm

# 2. Deploy with wadm
wadm app deploy recipe-system.yaml

# 3. Monitor deployment
wadm app get recipe-system --watch

# 4. Check all components are running
wash get inventory

# 5. Test the system
curl http://localhost:8080/api/recipes
```

### Example 4: Blue-Green Deployment

```bash
# 1. Deploy new version as separate app
wadm app deploy recipe-system-v2.yaml

# 2. Verify v2 is healthy
wadm app get recipe-system-v2
curl http://localhost:8081/health

# 3. Switch traffic (update load balancer or DNS)
# ...

# 4. Remove old version
wadm app delete recipe-system-v1
```

### Example 5: Edge Deployment

```bash
# Start wasmCloud on edge device
wash up --nats-remote-url nats://central-nats:4222

# Label the edge host
wash label host <host-id> location=edge device-type=raspberry-pi

# Deploy edge-specific manifest
cat > edge-wadm.yaml <<EOF
apiVersion: core.oam.dev/v1beta1
kind: Application
metadata:
  name: recipe-edge
spec:
  components:
    - name: recipe-cache
      type: component
      properties:
        image: file://./recipe_cache.wasm
      traits:
        - type: spreadscaler
          properties:
            instances: 1
            spread:
              - name: location
                requirements:
                  location: edge
EOF

wadm app deploy edge-wadm.yaml
```

## Guidelines

### Deployment Best Practices

1. **Environment Separation**
   - Use different NATS clusters for dev/staging/prod
   - Maintain separate wadm manifests per environment
   - Use environment-specific image tags
   - Never deploy dev builds to production

2. **Health Checks**
   ```bash
   # Check host health
   wash get hosts

   # Check component inventory
   wash get inventory

   # Check links are established
   wash get links

   # Monitor application status
   wadm app get <name>
   ```

3. **Rolling Updates**
   - Update wadm manifest with new image version
   - Apply updated manifest: `wadm app deploy wadm.yaml`
   - wadm automatically handles rolling update
   - Monitor status during rollout

4. **Rollback Strategy**
   ```bash
   # Keep previous versions in registry
   # Rollback by deploying old manifest
   wadm app deploy wadm-v1.0.0.yaml

   # Or update image in current manifest
   # Edit wadm.yaml to point to previous version
   wadm app deploy wadm.yaml
   ```

5. **Monitoring**
   ```bash
   # Watch application status
   watch -n 2 'wadm app get recipe-system'

   # Monitor host resources
   wash get hosts

   # Check component logs (if logging provider is running)
   wash logs <component-id>

   # Use wash spy for debugging
   wash spy
   ```

### Security Considerations

1. **Image Verification**
   - Use signed images
   - Verify image checksums
   - Pull from trusted registries only
   - Implement RBAC for registry access

2. **Network Security**
   - Use TLS for NATS connections
   - Implement network policies
   - Limit provider access
   - Use secrets management for credentials

3. **Access Control**
   - Restrict who can deploy
   - Use separate credentials per environment
   - Audit deployment changes
   - Implement approval workflows for production

### Performance Optimization

1. **Component Placement**
   ```bash
   # Label hosts by capability
   wash label host <id> cpu=high-performance
   wash label host <id> region=us-west

   # Use spread requirements in wadm
   spread:
     - name: cpu
       requirements:
         cpu: high-performance
   ```

2. **Scaling**
   - Start with conservative instance counts
   - Monitor resource usage
   - Scale based on metrics
   - Use spread to distribute load

3. **Caching**
   - Use local file:// images when possible
   - Cache OCI images on hosts
   - Implement CDN for image distribution

## Deployment Workflows

### CI/CD Pipeline

```bash
#!/bin/bash
# deploy.sh

set -e

# 1. Build
echo "Building components..."
cargo component build --release --target wasm32-wasip2

# 2. Test
echo "Running tests..."
cargo test

# 3. Push to registry
echo "Pushing to registry..."
VERSION=${GITHUB_SHA:0:7}
wash push ghcr.io/myorg/recipe-api:${VERSION} \
  ./target/wasm32-wasip2/release/recipe_api.wasm

wash push ghcr.io/myorg/recipe-api:latest \
  ./target/wasm32-wasip2/release/recipe_api.wasm

# 4. Update manifest
echo "Updating manifest..."
sed -i "s|image:.*|image: ghcr.io/myorg/recipe-api:${VERSION}|" wadm.yaml

# 5. Deploy
echo "Deploying to wasmCloud..."
wadm app deploy wadm.yaml

# 6. Verify
echo "Verifying deployment..."
sleep 10
wadm app get recipe-system | grep -q "Deployed" || exit 1

echo "Deployment successful!"
```

### Local Development Workflow

```bash
# Terminal 1: Start wasmCloud
wash up

# Terminal 2: Start development with auto-reload
cd recipe-api
wash dev

# Terminal 3: Test
curl http://localhost:8000/health

# Make code changes and watch auto-reload happen
```

## Troubleshooting

### Common Issues

**"component failed to start"**
```bash
# Check component is valid
wash inspect component.wasm

# Check host logs
wash get hosts
# Look for error messages

# Verify provider dependencies
wash get inventory
```

**"provider not found"**
```bash
# List running providers
wash get inventory | grep provider

# Start missing provider
wash start provider ghcr.io/wasmcloud/http-server:0.23.0
```

**"link failed"**
```bash
# Verify link configuration
wash get links

# Check interface compatibility
wash inspect component.wasm
wash inspect provider.wasm

# Recreate link
wash link del <component> <provider> --interface <interface>
wash link put <component> <provider> --interface <interface>
```

**"deployment stuck"**
```bash
# Check wadm status
wadm app get <name>

# Check host availability
wash get hosts

# Review host labels vs spread requirements
wash get hosts
# Compare with wadm manifest spread requirements

# Force redeployment
wadm app delete <name>
wadm app deploy wadm.yaml
```

### Debugging Commands

```bash
# Full system status
wash get hosts
wash get inventory
wash get links
wadm app list

# Inspect specific component
wash inspect component.wasm

# Monitor NATS traffic
wash spy

# Call component directly
wash call <component-id> <interface> <function> <args>
```

## Resources

- wasmCloud Deployment Guide: https://wasmcloud.com/docs/deployment/
- wash CLI Reference: https://wasmcloud.com/docs/cli/
- wadm Documentation: https://wasmcloud.com/docs/ecosystem/wadm/
- Production Checklist: https://wasmcloud.com/docs/deployment/production/
