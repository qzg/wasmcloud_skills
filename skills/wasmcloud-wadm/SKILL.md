---
name: wasmcloud-wadm
description: Create and manage wasmCloud Application Deployment Manager (wadm) manifests for orchestrating distributed applications
---

# WasmCloud WADM (Application Deployment Manager)

You are an expert in creating and managing wadm manifests for deploying and orchestrating distributed WebAssembly applications in wasmCloud.

## Core Capabilities

When helping users work with wadm, you should:

1. **Understanding wadm**
   - wadm is wasmCloud's declarative application deployment manager
   - Similar to Kubernetes deployments, but for WebAssembly components
   - Manages component lifecycle, scaling, and linking
   - Supports rolling updates and health checks
   - Works with OAM (Open Application Model) specification

2. **Manifest Structure**
   ```yaml
   apiVersion: core.oam.dev/v1beta1
   kind: Application
   metadata:
     name: application-name
     annotations:
       version: v0.1.0
       description: "Application description"
   spec:
     components:
       - name: component-name
         type: component
         properties:
           image: file://./component.wasm  # or registry URL
         traits:
           - type: spreadscaler
             properties:
               instances: 3
           - type: link
             properties:
               target: provider-name
               namespace: wasi
               package: keyvalue
               interfaces: [store]
   ```

3. **Component Configuration**
   - **image**: Component location (file, OCI registry, HTTP URL)
   - **id**: Optional component identifier
   - **config**: Configuration values passed to component
   - **secrets**: Reference to secrets for the component

4. **Traits (Deployment Behaviors)**
   - **spreadscaler**: Distribute instances across hosts
   - **daemonscaler**: Run exactly one instance per host
   - **link**: Connect components to providers or other components
   - **custom-traits**: Extend with custom behaviors

5. **Provider Configuration**
   ```yaml
   - name: keyvalue-provider
     type: capability
     properties:
       image: ghcr.io/wasmcloud/keyvalue-redis:0.23.0
       config:
         - name: redis-url
           properties:
             url: redis://localhost:6379
   ```

6. **Managing Applications**
   - Deploy: `wadm app deploy wadm.yaml`
   - List: `wadm app list`
   - Get status: `wadm app get <name>`
   - Delete: `wadm app delete <name>`
   - Update: Modify YAML and redeploy

## Examples

### Example 1: Simple HTTP Component

```yaml
apiVersion: core.oam.dev/v1beta1
kind: Application
metadata:
  name: hello-world
  annotations:
    version: v0.1.0
    description: "Simple HTTP hello world service"
spec:
  components:
    - name: hello-component
      type: component
      properties:
        image: file://./build/hello.wasm
      traits:
        - type: spreadscaler
          properties:
            instances: 1
        - type: link
          properties:
            target: httpserver
            namespace: wasi
            package: http
            interfaces: [incoming-handler]

    - name: httpserver
      type: capability
      properties:
        image: ghcr.io/wasmcloud/http-server:0.23.0
        config:
          - name: default-http
            properties:
              address: 0.0.0.0:8080
```

### Example 2: Complete Recipe Management System

```yaml
apiVersion: core.oam.dev/v1beta1
kind: Application
metadata:
  name: recipe-management-system
  annotations:
    version: v0.1.0
    description: "AI-powered recipe management with chat capabilities"
spec:
  components:
    # API Gateway Component
    - name: recipe-api
      type: component
      properties:
        image: file://./components/api/build/recipe_api.wasm
      traits:
        - type: spreadscaler
          properties:
            instances: 3
        - type: link
          properties:
            target: httpserver
            namespace: wasi
            package: http
            interfaces: [incoming-handler]
        - type: link
          properties:
            target: keyvalue
            namespace: wasi
            package: keyvalue
            interfaces: [store]

    # Chat/AI Component
    - name: recipe-chat
      type: component
      properties:
        image: file://./components/chat/build/recipe_chat.wasm
        config:
          - name: claude-api
            properties:
              model: claude-3-5-sonnet-20241022
      traits:
        - type: spreadscaler
          properties:
            instances: 2
        - type: link
          properties:
            target: keyvalue
            namespace: wasi
            package: keyvalue
            interfaces: [store]
        - type: link
          properties:
            target: httpclient
            namespace: wasi
            package: http
            interfaces: [outgoing-handler]

    # Recipe Calculator Component
    - name: recipe-calculator
      type: component
      properties:
        image: file://./components/calculator/build/calculator.wasm
      traits:
        - type: spreadscaler
          properties:
            instances: 2
        - type: link
          properties:
            target: keyvalue
            namespace: wasi
            package: keyvalue
            interfaces: [store]

    # HTTP Server Provider
    - name: httpserver
      type: capability
      properties:
        image: ghcr.io/wasmcloud/http-server:0.23.0
        config:
          - name: default-http
            properties:
              address: 0.0.0.0:8080

    # HTTP Client Provider
    - name: httpclient
      type: capability
      properties:
        image: ghcr.io/wasmcloud/http-client:0.12.1

    # Key-Value Store Provider (SQLite)
    - name: keyvalue
      type: capability
      properties:
        image: file://./providers/sqlite-provider.wasm
        config:
          - name: sqlite-config
            properties:
              database_path: ./data/recipes.db
```

### Example 3: Multi-Region Deployment

```yaml
apiVersion: core.oam.dev/v1beta1
kind: Application
metadata:
  name: recipe-global
  annotations:
    version: v0.1.0
    description: "Multi-region recipe service"
spec:
  components:
    - name: recipe-api-us
      type: component
      properties:
        image: file://./build/recipe_api.wasm
      traits:
        - type: spreadscaler
          properties:
            instances: 5
            spread:
              - name: region
                requirements:
                  region: us-east
        - type: link
          properties:
            target: keyvalue-us
            namespace: wasi
            package: keyvalue
            interfaces: [store]

    - name: recipe-api-eu
      type: component
      properties:
        image: file://./build/recipe_api.wasm
      traits:
        - type: spreadscaler
          properties:
            instances: 5
            spread:
              - name: region
                requirements:
                  region: eu-west
        - type: link
          properties:
            target: keyvalue-eu
            namespace: wasi
            package: keyvalue
            interfaces: [store]

    - name: keyvalue-us
      type: capability
      properties:
        image: ghcr.io/wasmcloud/keyvalue-redis:0.23.0
        config:
          - name: redis-us
            properties:
              url: redis://us-redis:6379

    - name: keyvalue-eu
      type: capability
      properties:
        image: ghcr.io/wasmcloud/keyvalue-redis:0.23.0
        config:
          - name: redis-eu
            properties:
              url: redis://eu-redis:6379
```

### Example 4: Development vs Production

```yaml
# wadm-dev.yaml
apiVersion: core.oam.dev/v1beta1
kind: Application
metadata:
  name: recipe-dev
spec:
  components:
    - name: recipe-api
      type: component
      properties:
        image: file://./target/wasm32-wasip2/debug/recipe_api.wasm
      traits:
        - type: spreadscaler
          properties:
            instances: 1  # Single instance for dev

# wadm-prod.yaml
apiVersion: core.oam.dev/v1beta1
kind: Application
metadata:
  name: recipe-prod
spec:
  components:
    - name: recipe-api
      type: component
      properties:
        image: ghcr.io/myorg/recipe-api:v1.2.3  # OCI registry
      traits:
        - type: spreadscaler
          properties:
            instances: 10  # Scale up for production
            spread:
              - name: availability-zone
                weight: 100
```

## Guidelines

### Best Practices

1. **Application Design**
   - One manifest per logical application
   - Use meaningful component names
   - Group related components together
   - Separate concerns (API, business logic, storage)

2. **Scaling Strategy**
   - Use `spreadscaler` for stateless components
   - Use `daemonscaler` for monitoring/logging
   - Consider host resources when setting instance counts
   - Start small and scale based on metrics

3. **Linking**
   - Link components only to required providers
   - Use specific interface names, not wildcards
   - Verify provider supports required interfaces
   - Document link dependencies

4. **Configuration Management**
   - Use config for environment-specific values
   - Never hardcode secrets in manifests
   - Use descriptive config names
   - Validate config before deployment

5. **Versioning**
   - Use semantic versioning for applications
   - Tag component images with versions
   - Keep old versions for rollback
   - Document breaking changes

### Deployment Workflow

1. **Development**
   ```bash
   # Build component
   cargo component build --release

   # Deploy with wadm
   wadm app deploy wadm-dev.yaml

   # Check status
   wadm app get recipe-dev

   # Watch logs
   wash get inventory
   ```

2. **Testing**
   ```bash
   # Deploy to test environment
   wadm app deploy wadm-test.yaml

   # Run integration tests
   ./run-tests.sh

   # Check health
   curl http://localhost:8080/health
   ```

3. **Production**
   ```bash
   # Push image to registry
   wash push ghcr.io/myorg/recipe-api:v1.0.0 ./component.wasm

   # Deploy to production
   wadm app deploy wadm-prod.yaml

   # Monitor deployment
   wadm app get recipe-prod --watch
   ```

### Monitoring and Debugging

- **Check app status**: `wadm app get <name>`
- **List all apps**: `wadm app list`
- **View history**: `wadm app history <name>`
- **Check component health**: `wash get inventory`
- **View links**: `wash get links`
- **Inspect component**: `wash inspect component.wasm`

### Common Patterns

**Blue-Green Deployment**
```yaml
# Deploy new version with different name
# Switch traffic when ready
# Delete old version
```

**Canary Deployment**
```yaml
components:
  - name: recipe-api-stable
    traits:
      - type: spreadscaler
        properties:
          instances: 9

  - name: recipe-api-canary
    traits:
      - type: spreadscaler
        properties:
          instances: 1  # 10% traffic
```

**Service Mesh**
```yaml
# Link components through intermediary components
components:
  - name: api-gateway
    # Links to backend services
  - name: backend-service
    # Links to data stores
```

## Common Issues

**"component failed to start"**
- Check component image path is correct
- Verify component is valid: `wash inspect component.wasm`
- Check provider is running and linked
- Review wasmCloud host logs

**"link failed"**
- Verify provider supports the interface
- Check namespace, package, and interface names match
- Ensure provider is running
- Verify link configuration

**"spread failed"**
- Check host labels match spread requirements
- Verify enough hosts are available
- Review resource constraints

**"config not found"**
- Verify config name matches
- Check config was created before deployment
- Ensure config properties are valid

## Advanced Features

### Policy and Governance
```yaml
traits:
  - type: policy
    properties:
      allow-outbound: ["*.api.example.com"]
      max-memory-mb: 128
      max-cpu-percent: 50
```

### Custom Health Checks
```yaml
traits:
  - type: health
    properties:
      http-endpoint: /health
      interval-seconds: 30
      timeout-seconds: 5
```

### Auto-scaling (future feature)
```yaml
traits:
  - type: autoscaler
    properties:
      min-instances: 2
      max-instances: 20
      target-cpu-percent: 70
```

## Resources

- wadm Documentation: https://wasmcloud.com/docs/ecosystem/wadm/
- wadm GitHub: https://github.com/wasmCloud/wadm
- OAM Specification: https://oam.dev/
- wasmCloud Examples: https://github.com/wasmCloud/wasmCloud/tree/main/examples
