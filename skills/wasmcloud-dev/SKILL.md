---
name: wasmcloud-dev
description: Development workflow and best practices for building wasmCloud applications
---

# WasmCloud Development Workflow

You are an expert in wasmCloud development workflows, providing guidance on rapid iteration, testing, and debugging.

## Core Capabilities

When helping users with wasmCloud development, you should:

1. **Development Environment Setup**
   ```bash
   # Install all tools
   cargo install wash-cli cargo-component wkg --locked
   rustup target add wasm32-wasip2

   # Verify installation
   wash --version
   cargo component --version

   # Start wasmCloud
   wash up
   ```

2. **Rapid Development with wash dev**

   The `wash dev` command provides hot-reloading and automatic deployment:

   ```bash
   # Start development mode
   wash dev

   # What it does:
   # 1. Starts a local wasmCloud host (if not running)
   # 2. Builds your component
   # 3. Deploys it
   # 4. Watches for file changes
   # 5. Automatically rebuilds and redeploys on changes
   ```

   **Configuration** (`wasmcloud.toml`):
   ```toml
   name = "recipe-api"
   language = "rust"
   type = "component"

   [component]
   claims = ["wasmcloud:httpserver"]
   wit_world = "recipe-api"

   [[dev]]
   # Custom build command
   build_command = "cargo component build --release"
   # Custom deploy manifest
   deploy_manifest = "wadm.yaml"
   ```

3. **Project Structure**
   ```
   recipe-api/
   ├── Cargo.toml
   ├── wasmcloud.toml      # wash dev configuration
   ├── wadm.yaml            # Deployment manifest
   ├── wit/
   │   ├── deps/            # External WIT dependencies
   │   └── world.wit        # Interface definitions
   ├── src/
   │   ├── lib.rs           # Main component code
   │   └── handlers/        # Request handlers
   ├── tests/
   │   ├── integration/     # Integration tests
   │   └── unit/            # Unit tests
   └── target/
       └── wasm32-wasip2/   # Build output
   ```

4. **Build Process**
   ```bash
   # Debug build (faster, larger)
   cargo component build

   # Release build (slower, optimized)
   cargo component build --release --target wasm32-wasip2

   # Check build output
   ls -lh target/wasm32-wasip2/release/*.wasm

   # Inspect the component
   wash inspect target/wasm32-wasip2/release/recipe_api.wasm
   ```

5. **Testing Strategy**

   **Unit Tests** (in Rust code):
   ```rust
   #[cfg(test)]
   mod tests {
       use super::*;

       #[test]
       fn test_parse_recipe() {
           let recipe = Recipe::new("Cookies", vec![]);
           assert_eq!(recipe.name, "Cookies");
       }
   }
   ```

   **Integration Tests** (with wash):
   ```bash
   # Start test environment
   wash up

   # Deploy component
   wash dev &

   # Run tests against deployed component
   curl http://localhost:8000/api/recipes
   cargo test --test integration
   ```

   **Component Tests** (WIT-level):
   ```rust
   // tests/integration/component_test.rs
   use wasmtime::component::*;

   #[test]
   fn test_component_interface() {
       let engine = Engine::default();
       let component = Component::from_file(&engine, "component.wasm")?;
       // Test component exports/imports
   }
   ```

6. **Debugging Tools**

   **wash spy** - Intercept and inspect messages:
   ```bash
   wash spy

   # Filter by component
   wash spy --component recipe-api

   # Watch specific operations
   wash spy --operation wasi:http/incoming-handler.handle
   ```

   **wash call** - Manually invoke component functions:
   ```bash
   wash call <component-id> \
     wasi:http/incoming-handler \
     handle \
     '{"method": "GET", "path": "/health"}'
   ```

   **Logging**:
   ```rust
   use wasi::logging::logging::*;

   log(Level::Info, "recipe-api", "Processing request");
   log(Level::Debug, "recipe-api", &format!("Data: {:?}", data));
   log(Level::Error, "recipe-api", "Failed to process");
   ```

## Examples

### Example 1: Complete Development Workflow

```bash
# 1. Create new component
wash new component recipe-api --template-name hello-world-rust
cd recipe-api

# 2. Define WIT interfaces
cat > wit/world.wit <<EOF
package wasmcloud:recipe@0.1.0;

interface recipe-api {
    get-recipe: func(id: string) -> string;
}

world recipe-service {
    import wasi:http/incoming-handler@0.2.0;
    import wasi:keyvalue/store@0.2.0;
    export wasi:http/incoming-handler;
}
EOF

# 3. Implement component
# Edit src/lib.rs

# 4. Start development mode
wash dev

# 5. Test in another terminal
curl http://localhost:8000/api/recipes/1

# 6. Make changes to src/lib.rs
# wash dev automatically rebuilds and redeploys

# 7. Test again to see changes
curl http://localhost:8000/api/recipes/1
```

### Example 2: Multi-Component Development

```bash
# Terminal 1: API Component
cd components/api
wash dev --port 8000

# Terminal 2: Chat Component
cd components/chat
wash dev --port 8001

# Terminal 3: Calculator Component
cd components/calculator
wash dev --port 8002

# Terminal 4: Test full system
./scripts/test-integration.sh
```

### Example 3: Debugging with wash spy

```bash
# Terminal 1: Start component
wash dev

# Terminal 2: Start spy
wash spy

# Terminal 3: Make requests
curl http://localhost:8000/api/recipes

# Terminal 2 shows:
# → Incoming HTTP request
#   Method: GET
#   Path: /api/recipes
# → Component invocation
#   Component: recipe-api
#   Interface: wasi:http/incoming-handler
#   Function: handle
# → Keyvalue operation
#   Operation: get
#   Key: "recipes:list"
# ← HTTP response
#   Status: 200
#   Body length: 1234 bytes
```

### Example 4: Performance Profiling

```rust
// Add timing to your component
use wasi::clocks::wall_clock::now;

fn handle_request(req: Request) -> Response {
    let start = now();

    // ... handle request ...

    let duration = now() - start;
    log(
        Level::Info,
        "perf",
        &format!("Request handled in {}ms", duration.milliseconds())
    );

    response
}
```

```bash
# Monitor performance
wash spy | grep "perf"

# Analyze logs
wash logs recipe-api | jq '.[] | select(.context == "perf")'
```

### Example 5: CI/CD Integration

```yaml
# .github/workflows/build.yml
name: Build and Test

on: [push, pull_request]

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3

      - name: Setup Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          target: wasm32-wasip2

      - name: Install wash
        run: cargo install wash-cli --locked

      - name: Build component
        run: cargo component build --release

      - name: Inspect component
        run: wash inspect target/wasm32-wasip2/release/*.wasm

      - name: Start wasmCloud
        run: wash up -d

      - name: Deploy component
        run: wadm app deploy wadm.yaml

      - name: Run tests
        run: |
          sleep 5  # Wait for deployment
          cargo test
          ./scripts/integration-test.sh

      - name: Push to registry
        if: github.ref == 'refs/heads/main'
        run: |
          wash push ghcr.io/${{ github.repository }}:latest \
            target/wasm32-wasip2/release/*.wasm
```

## Guidelines

### Best Practices

1. **Code Organization**
   - Keep components small and focused
   - Separate business logic from I/O
   - Use modules to organize code
   - Define clear interfaces in WIT

2. **Development Workflow**
   - Use `wash dev` for rapid iteration
   - Write tests before implementation (TDD)
   - Commit working code frequently
   - Keep dependencies minimal

3. **Testing Strategy**
   - Unit tests for business logic
   - Integration tests for component interactions
   - Contract tests for WIT interfaces
   - End-to-end tests for full workflows

4. **Error Handling**
   ```rust
   // Good: Proper error types
   enum Error {
       NotFound(String),
       InvalidInput(String),
       StorageError(String),
   }

   fn get_recipe(id: &str) -> Result<Recipe, Error> {
       // ...
   }

   // Bad: Generic errors
   fn get_recipe(id: &str) -> Result<Recipe, String> {
       // ...
   }
   ```

5. **Performance**
   - Profile with `wash spy`
   - Optimize hot paths
   - Use appropriate data structures
   - Minimize allocations
   - Batch operations when possible

6. **Security**
   - Validate all inputs
   - Use typed interfaces
   - Never trust external data
   - Use secrets management
   - Audit dependencies

### Development Tips

**Hot Reloading**
- `wash dev` automatically rebuilds on file changes
- Changes to WIT files trigger rebuild
- Changes to wadm.yaml trigger redeployment
- Use `.washignore` to exclude files

**Fast Builds**
```toml
# Cargo.toml - optimize for dev builds
[profile.dev]
opt-level = 0
debug = true
split-debuginfo = "unpacked"

[profile.release]
opt-level = "z"  # Optimize for size
lto = true
codegen-units = 1
```

**Component Size**
```bash
# Check component size
ls -lh target/wasm32-wasip2/release/*.wasm

# Optimize with wasm-opt
wasm-opt -Oz input.wasm -o output.wasm

# Strip debug info
wasm-strip component.wasm
```

**Debugging Builds**
```bash
# Enable verbose output
RUST_LOG=debug wash dev

# Check what's being built
cargo build --verbose

# See generated bindings
cargo expand
```

### Common Workflows

**Feature Development**
```bash
# 1. Create feature branch
git checkout -b feature/recipe-search

# 2. Update WIT interface
# Edit wit/world.wit

# 3. Implement feature
# Edit src/lib.rs

# 4. Test locally
wash dev
# Test in browser/curl

# 5. Add tests
# Edit tests/

# 6. Run tests
cargo test

# 7. Commit and push
git add .
git commit -m "Add recipe search"
git push
```

**Bug Fix**
```bash
# 1. Reproduce bug
wash dev
curl http://localhost:8000/api/recipes/invalid

# 2. Add failing test
# Edit tests/

# 3. Fix bug
# Edit src/lib.rs

# 4. Verify fix
cargo test
wash dev
curl http://localhost:8000/api/recipes/invalid

# 5. Commit
git commit -m "Fix: Handle invalid recipe IDs"
```

**Dependency Update**
```bash
# 1. Update dependencies
cargo update

# 2. Test build
cargo component build

# 3. Run all tests
cargo test

# 4. Test deployment
wash dev

# 5. Run integration tests
./scripts/test.sh

# 6. Commit if successful
git commit -m "Update dependencies"
```

## Troubleshooting

### Common Issues

**"component failed to build"**
```bash
# Check Rust version
rustc --version

# Verify target is installed
rustup target list --installed | grep wasm32-wasip2

# Clean build
cargo clean
cargo component build

# Check for syntax errors
cargo check
```

**"wash dev not reloading"**
```bash
# Restart wash dev
# Ctrl+C and restart

# Check wasmcloud.toml
cat wasmcloud.toml

# Verify file watcher is working
# Make a change and check console output
```

**"link failed during dev"**
```bash
# Check wadm.yaml configuration
cat wadm.yaml

# Verify providers are running
wash get inventory

# Check link configuration
wash get links

# Restart with clean state
wash down
wash up
wash dev
```

**"test failures"**
```bash
# Run specific test
cargo test test_name -- --nocapture

# Run with logging
RUST_LOG=debug cargo test

# Check test output
cargo test -- --show-output
```

### Debugging Techniques

**Add extensive logging**
```rust
use wasi::logging::logging::*;

fn process(data: &str) {
    log(Level::Debug, "process", &format!("Input: {}", data));

    let result = transform(data);
    log(Level::Debug, "process", &format!("Transformed: {:?}", result));

    match validate(&result) {
        Ok(_) => log(Level::Info, "process", "Validation passed"),
        Err(e) => log(Level::Error, "process", &format!("Validation failed: {}", e)),
    }
}
```

**Use wash spy for tracing**
```bash
# Trace all operations
wash spy --verbose

# Filter by component
wash spy --component recipe-api

# Save to file for analysis
wash spy > spy-output.log
```

**Inspect component details**
```bash
# View component metadata
wash inspect component.wasm

# View imports/exports
wash inspect --json component.wasm | jq '.imports, .exports'

# Verify WIT world
wasm-tools component wit component.wasm
```

## Resources

- wash CLI Guide: https://wasmcloud.com/docs/cli/
- Development Guide: https://wasmcloud.com/docs/developer/
- wash dev Documentation: https://wasmcloud.com/docs/developer/workflow/
- Testing Guide: https://wasmcloud.com/docs/developer/testing/
- Example Projects: https://github.com/wasmCloud/wasmCloud/tree/main/examples
