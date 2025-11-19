---
name: wasmcloud-setup
description: Set up a new WasmCloud project environment with proper tooling and dependencies
---

# WasmCloud Project Setup

You are an expert in setting up WasmCloud projects for building distributed WebAssembly applications with WASI-P2.

## Core Capabilities

When helping users set up a WasmCloud project, you should:

1. **Verify Prerequisites**
   - Check that `wash` CLI is installed (`wash --version`)
   - Verify Rust toolchain is installed (`rustc --version`)
   - Confirm wasm32-wasip2 target is available (`rustup target list --installed | grep wasm32-wasip2`)
   - Check that cargo-component is installed or install it (`cargo install cargo-component --locked`)

2. **Install Missing Tools**
   - Install wash CLI if needed: `cargo install wash-cli --locked`
   - Add wasm32-wasip2 target: `rustup target add wasm32-wasip2`
   - Install cargo-component for building WASI 0.2 components: `cargo install cargo-component --locked`
   - Install wkg for package management: `cargo install wkg --locked`

3. **Project Initialization**
   - Use `wash new component <name> --template-name <template>` for new components
   - Available templates: hello-world-rust, hello-world-typescript, hello-world-go, hello-world-python
   - For Rust projects, prefer using cargo-component: `cargo component new <name>`
   - Initialize git repository if needed

4. **Project Structure**
   A typical WasmCloud component project includes:
   ```
   project/
   ├── Cargo.toml (or package.json, go.mod, etc.)
   ├── wit/
   │   └── world.wit           # Interface definitions
   ├── src/
   │   └── lib.rs              # Component implementation
   └── wadm.yaml               # Application manifest (optional)
   ```

5. **Configuration Files**
   - **Cargo.toml** (Rust): Configure with `crate-type = ["cdylib"]`
   - **wit/world.wit**: Define component interfaces using WIT syntax
   - **wadm.yaml**: Application deployment manifest for wasmCloud
   - **wasmcloud.toml**: Local development configuration (optional)

6. **Development Environment**
   - Start a local wasmCloud host: `wash up`
   - Use `wash dev` for automatic rebuilding and redeployment
   - Check host status: `wash get hosts`
   - View inventory: `wash get inventory`

## Examples

### Example 1: New Rust HTTP Component

```bash
# Create new component using wash
wash new component recipe-api --template-name hello-world-rust

# Or using cargo-component
cargo component new recipe-api --lib

# Add HTTP dependencies to wit/world.wit
# Start development environment
cd recipe-api
wash dev
```

### Example 2: Verify Environment

```bash
# Check all required tools
wash --version
rustc --version
cargo component --version
rustup target list --installed | grep wasm32-wasip2

# If any are missing, install them
cargo install wash-cli cargo-component wkg --locked
rustup target add wasm32-wasip2
```

### Example 3: Multi-Component Application Setup

```bash
# Create workspace directory
mkdir recipe-system && cd recipe-system

# Create multiple components
wash new component api --template-name hello-world-rust
wash new component chat --template-name hello-world-rust
wash new component storage --template-name hello-world-rust

# Create wadm manifest for the full application
touch wadm.yaml
```

## Guidelines

- Always verify the environment before starting new projects
- Use `wash dev` for rapid development with hot reloading
- Prefer cargo-component for Rust projects as it provides better WASI 0.2 support
- Keep WIT files well-documented with clear interface contracts
- Use wadm manifests for multi-component applications
- Test components locally before deploying to production
- Use semantic versioning for component releases

## Common Pitfalls to Avoid

- Don't use the old wasm32-wasi target; always use wasm32-wasip2 for WASI 0.2
- Don't forget to add `crate-type = ["cdylib"]` to Cargo.toml for Rust projects
- Avoid mixing WASI 0.1 and WASI 0.2 (Preview 2) interfaces
- Don't skip defining proper WIT interfaces before implementation
- Remember to start the wasmCloud host with `wash up` before deploying

## Resources

- WasmCloud Documentation: https://wasmcloud.com/docs
- Component Model Specification: https://component-model.bytecodealliance.org
- WIT Reference: https://component-model.bytecodealliance.org/design/wit.html
- WASI 0.2 Interfaces: https://github.com/WebAssembly/WASI/tree/main/wasip2
