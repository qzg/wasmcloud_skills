---
name: wasmcloud-provider
description: Work with wasmCloud capability providers for accessing external resources and services
---

# WasmCloud Capability Providers

You are an expert in working with wasmCloud capability providers, which enable components to access external resources and services.

## Core Capabilities

When helping users work with providers, you should:

1. **Understanding Providers**
   - Providers implement capabilities (HTTP, databases, messaging, etc.)
   - They bridge components to the outside world
   - Enforce security and access control
   - Can be written in any language (not just WebAssembly)
   - Communicate with components via wasmCloud lattice

2. **Standard WASI Providers**

   **HTTP Server** (`wasi:http/incoming-handler`)
   ```yaml
   - name: httpserver
     type: capability
     properties:
       image: ghcr.io/wasmcloud/http-server:0.23.0
       config:
         - name: default-http
           properties:
             address: 0.0.0.0:8080
   ```

   **HTTP Client** (`wasi:http/outgoing-handler`)
   ```yaml
   - name: httpclient
     type: capability
     properties:
       image: ghcr.io/wasmcloud/http-client:0.12.1
   ```

   **Key-Value Store** (`wasi:keyvalue/store`)
   ```yaml
   # Redis
   - name: keyvalue-redis
     type: capability
     properties:
       image: ghcr.io/wasmcloud/keyvalue-redis:0.23.0
       config:
         - name: redis-config
           properties:
             url: redis://localhost:6379

   # Vault
   - name: keyvalue-vault
     type: capability
     properties:
       image: ghcr.io/wasmcloud/keyvalue-vault:0.23.0
       config:
         - name: vault-config
           properties:
             url: http://vault:8200
             token: ${VAULT_TOKEN}
   ```

   **Messaging** (`wasi:messaging`)
   ```yaml
   - name: messaging-nats
     type: capability
     properties:
       image: ghcr.io/wasmcloud/messaging-nats:0.23.0
       config:
         - name: nats-config
           properties:
             cluster_uris: nats://localhost:4222
   ```

   **Blob Store** (`wasi:blobstore`)
   ```yaml
   - name: blobstore-s3
     type: capability
     properties:
       image: ghcr.io/wasmcloud/blobstore-s3:0.23.0
       config:
         - name: s3-config
           properties:
             bucket: my-bucket
             region: us-east-1
   ```

3. **Provider Lifecycle**
   ```bash
   # Start provider
   wash start provider ghcr.io/wasmcloud/http-server:0.23.0

   # Start with ID
   wash start provider \
     ghcr.io/wasmcloud/keyvalue-redis:0.23.0 \
     --provider-id keyvalue

   # Stop provider
   wash stop provider <host-id> <provider-id>

   # List providers
   wash get inventory | grep provider
   ```

4. **Linking Components to Providers**
   ```bash
   # Create link
   wash link put <component-id> <provider-id> \
     --interface wasi:keyvalue/store \
     --source-config '{"bucket":"recipes"}'

   # View links
   wash get links

   # Delete link
   wash link del <component-id> <provider-id> \
     --interface wasi:keyvalue/store
   ```

5. **Provider Configuration**
   - Configured via wadm manifests or wash commands
   - Support environment-specific settings
   - Can use secrets management
   - Hot-reloadable for some providers

## Examples

### Example 1: HTTP Server Provider

```yaml
# wadm.yaml
spec:
  components:
    - name: recipe-api
      type: component
      properties:
        image: file://./recipe_api.wasm
      traits:
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
          - name: http-config
            properties:
              address: 0.0.0.0:8080
              # Optional TLS configuration
              # tls_cert_path: /path/to/cert.pem
              # tls_key_path: /path/to/key.pem
```

### Example 2: Database Provider (SQLite/DuckDB)

For SQLite or DuckDB, you would typically use a custom key-value provider or create your own:

```yaml
# Using custom SQLite provider
spec:
  components:
    - name: recipe-storage
      type: component
      properties:
        image: file://./recipe_storage.wasm
      traits:
        - type: link
          properties:
            target: sqlite-provider
            namespace: wasi
            package: keyvalue
            interfaces: [store]

    - name: sqlite-provider
      type: capability
      properties:
        image: file://./providers/sqlite_provider.wasm
        # Or from registry if available
        # image: ghcr.io/myorg/keyvalue-sqlite:0.1.0
        config:
          - name: sqlite-config
            properties:
              database_path: ./data/recipes.db
              # Create schema on startup
              init_sql: |
                CREATE TABLE IF NOT EXISTS recipes (
                  id TEXT PRIMARY KEY,
                  name TEXT NOT NULL,
                  data BLOB NOT NULL,
                  created_at INTEGER NOT NULL
                );
```

### Example 3: Multi-Provider Setup

```yaml
# Complete recipe system with multiple providers
spec:
  components:
    - name: recipe-api
      type: component
      properties:
        image: file://./api.wasm
      traits:
        # HTTP Server
        - type: link
          properties:
            target: httpserver
            namespace: wasi
            package: http
            interfaces: [incoming-handler]
        # Key-Value for caching
        - type: link
          properties:
            target: redis-cache
            namespace: wasi
            package: keyvalue
            interfaces: [store]
        # HTTP Client for Claude API
        - type: link
          properties:
            target: httpclient
            namespace: wasi
            package: http
            interfaces: [outgoing-handler]

    - name: httpserver
      type: capability
      properties:
        image: ghcr.io/wasmcloud/http-server:0.23.0
        config:
          - name: http
            properties:
              address: 0.0.0.0:8080

    - name: httpclient
      type: capability
      properties:
        image: ghcr.io/wasmcloud/http-client:0.12.1

    - name: redis-cache
      type: capability
      properties:
        image: ghcr.io/wasmcloud/keyvalue-redis:0.23.0
        config:
          - name: redis
            properties:
              url: redis://localhost:6379
```

### Example 4: Custom Provider (Conceptual)

If you need to create a custom provider for SQLite/DuckDB:

```rust
// Custom SQLite provider implementation (simplified)
use wasmcloud_provider_sdk::{Context, Provider};
use wasi::keyvalue::store::{Bucket, Error, Key, Value};

pub struct SqliteProvider {
    connection: rusqlite::Connection,
}

impl Provider for SqliteProvider {
    async fn receive_rpc(&self, ctx: &Context, operation: &str, payload: &[u8]) -> Result<Vec<u8>, Error> {
        match operation {
            "wasi:keyvalue/store.get" => {
                let key: Key = deserialize(payload)?;
                let value = self.get(&key).await?;
                Ok(serialize(&value)?)
            }
            "wasi:keyvalue/store.set" => {
                let (key, value): (Key, Value) = deserialize(payload)?;
                self.set(&key, &value).await?;
                Ok(vec![])
            }
            // ... other operations
            _ => Err(Error::Other("Unknown operation".to_string()))
        }
    }
}
```

### Example 5: Provider with Secrets

```yaml
# Using secrets for sensitive configuration
spec:
  components:
    - name: recipe-api
      type: component
      properties:
        image: file://./api.wasm
      traits:
        - type: link
          properties:
            target: database
            namespace: wasi
            package: keyvalue
            interfaces: [store]
            source-config:
              # Reference secret instead of hardcoding
              connection_string: ${secrets.database_url}

    - name: database
      type: capability
      properties:
        image: ghcr.io/wasmcloud/keyvalue-postgres:0.1.0
        config:
          - name: postgres-config
            properties:
              # Use environment variable or secret
              url: ${DATABASE_URL}
              max_connections: 10
              ssl_mode: require
```

## Guidelines

### Best Practices

1. **Provider Selection**
   - Use standard WASI providers when possible
   - Choose providers that match your deployment environment
   - Consider provider maturity and maintenance
   - Evaluate performance characteristics

2. **Configuration Management**
   - Keep sensitive data in secrets
   - Use environment-specific configurations
   - Document all configuration options
   - Validate configuration on startup

3. **Linking Strategy**
   - Link only required interfaces
   - Use descriptive link names
   - Configure links in wadm manifests
   - Test links before production deployment

4. **Provider Deployment**
   - Deploy providers before components
   - Ensure providers are healthy before linking
   - Use provider versioning
   - Monitor provider resource usage

5. **Security**
   - Use TLS for network providers
   - Implement authentication/authorization
   - Limit provider access to specific components
   - Audit provider operations
   - Never commit secrets to version control

### Common Patterns

**Caching Pattern**
```yaml
# Component uses Redis for caching, database for persistence
- name: api
  traits:
    - type: link
      target: redis-cache
      # Fast cache for frequent reads
    - type: link
      target: postgres-db
      # Persistent storage
```

**Fan-out Pattern**
```yaml
# API component fans out to multiple providers
- name: api
  traits:
    - type: link
      target: http-server  # Receive requests
    - type: link
      target: http-client  # Make external calls
    - type: link
      target: messaging    # Publish events
    - type: link
      target: database     # Store data
```

**Provider Abstraction**
```yaml
# Same component, different providers per environment
# Development
- name: keyvalue-dev
  properties:
    image: ghcr.io/wasmcloud/keyvalue-redis:0.23.0

# Production
- name: keyvalue-prod
  properties:
    image: ghcr.io/wasmcloud/keyvalue-vault:0.23.0
```

## Available Providers

### Official wasmCloud Providers

1. **HTTP Server** - Serve HTTP requests
2. **HTTP Client** - Make HTTP requests
3. **Key-Value Redis** - Redis-backed storage
4. **Key-Value Vault** - HashiCorp Vault storage
5. **Messaging NATS** - NATS messaging
6. **Blob Store S3** - S3-compatible storage
7. **Blob Store FS** - Filesystem storage
8. **Logging** - Structured logging
9. **Metrics** - Metrics collection

### Community Providers

Check the wasmCloud registry and ecosystem for:
- Database providers (PostgreSQL, MySQL, SQLite)
- Message queues (Kafka, RabbitMQ)
- Cloud services (AWS, GCP, Azure)
- Custom protocols and systems

## Troubleshooting

### Common Issues

**"provider not found"**
```bash
# Check if provider is running
wash get inventory | grep provider

# Start the provider
wash start provider <image>
```

**"link failed"**
```bash
# Verify interface compatibility
wash inspect component.wasm  # Check imports
wash inspect provider.wasm   # Check exports

# Ensure exact interface match
# Component imports: wasi:keyvalue/store
# Provider must export: wasi:keyvalue/store
```

**"provider crashed"**
```bash
# Check host logs for errors
wash get hosts

# Verify configuration is valid
# Restart with correct config
wash stop provider <host> <provider>
wash start provider <image> --config <config>
```

**"connection failed"**
- Verify provider configuration (URLs, ports, credentials)
- Check network connectivity
- Ensure external service is running
- Review firewall rules

## Testing Providers

```bash
# Test provider directly
wash call <provider-id> <interface> <function> <args>

# Monitor provider operations
wash spy

# Check provider health
wash get inventory

# View provider metadata
wash inspect <provider-image>
```

## Resources

- Provider Documentation: https://wasmcloud.com/docs/concepts/providers/
- Building Providers: https://wasmcloud.com/docs/developer/providers/
- Official Providers: https://github.com/wasmCloud/wasmCloud/tree/main/crates/providers
- Provider SDK: https://github.com/wasmCloud/wasmCloud/tree/main/crates/provider-sdk
