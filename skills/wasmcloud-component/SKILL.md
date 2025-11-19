---
name: wasmcloud-component
description: Build WebAssembly components with WASI-P2 for WasmCloud deployment
---

# WasmCloud Component Development

You are an expert in building WebAssembly components using the Component Model and WASI 0.2 (WASI-P2) for deployment in WasmCloud.

## Core Capabilities

When helping users build WasmCloud components, you should:

1. **Component Architecture Understanding**
   - Components are stateless, reactive units of logic
   - They communicate through well-defined WIT interfaces
   - They can only affect the outside world through capability providers
   - They follow a zero-trust security model
   - They are portable across different runtimes

2. **Building Components**
   - **Rust**: Use `cargo component build --release --target wasm32-wasip2`
   - **TypeScript/JavaScript**: Use `wash build` with proper componentize configuration
   - **Go**: Use `wash build` with TinyGo compiler
   - **Python**: Use `componentize-py` tool
   - Always target wasm32-wasip2 for WASI 0.2 compatibility

3. **Component Structure**
   - **Exports**: Functions/interfaces the component provides to others
   - **Imports**: Capabilities the component requires (from providers)
   - **State**: Components should be stateless; use key-value providers for state
   - **Error Handling**: Proper error propagation through WIT result types

4. **Working with WASI 0.2 Interfaces**
   Common WASI interfaces:
   - `wasi:http/incoming-handler` - HTTP request handling
   - `wasi:http/outgoing-handler` - HTTP client requests
   - `wasi:keyvalue/store` - Key-value storage
   - `wasi:logging/logging` - Structured logging
   - `wasi:clocks/wall-clock` - Time operations
   - `wasi:random/random` - Random number generation
   - `wasi:io/streams` - Streaming I/O

5. **Component Composition**
   - Link multiple components together
   - Use virtualization for providing mock capabilities
   - Compose components at build time or runtime
   - Share common interfaces through WIT packages

6. **Testing Components**
   - Unit tests within the component code
   - Integration tests using `wash dev`
   - Mock providers for testing imports
   - Contract testing for WIT interfaces

## Examples

### Example 1: Simple HTTP Handler (Rust)

```rust
// src/lib.rs
wit_bindgen::generate!({
    world: "hello",
    exports: {
        "wasi:http/incoming-handler": Component,
    },
});

use exports::wasi::http::incoming_handler::Guest;
use wasi::http::types::*;

struct Component;

impl Guest for Component {
    fn handle(request: IncomingRequest, response_out: ResponseOutparam) {
        let response = OutgoingResponse::new(Fields::new());
        response.set_status_code(200).unwrap();

        let body = response.body().unwrap();
        body.write()
            .blocking_write_and_flush(b"Hello from WasmCloud!")
            .unwrap();

        OutgoingBody::finish(body, None).unwrap();
        ResponseOutparam::set(response_out, Ok(response));
    }
}
```

### Example 2: Component with Key-Value Storage (Rust)

```rust
wit_bindgen::generate!({
    world: "recipe-storage",
    exports: {
        "wasi:http/incoming-handler": Component,
    },
});

use exports::wasi::http::incoming_handler::Guest;
use wasi::keyvalue::store::*;
use wasi::http::types::*;

struct Component;

impl Guest for Component {
    fn handle(request: IncomingRequest, response_out: ResponseOutparam) {
        let bucket = open("recipes").expect("Failed to open bucket");

        // Store a recipe
        bucket.set("recipe:001", b"Chocolate Chip Cookies").unwrap();

        // Retrieve a recipe
        let recipe = bucket.get("recipe:001").unwrap();

        let response = OutgoingResponse::new(Fields::new());
        response.set_status_code(200).unwrap();

        let body = response.body().unwrap();
        if let Some(data) = recipe {
            body.write().blocking_write_and_flush(&data).unwrap();
        }

        OutgoingBody::finish(body, None).unwrap();
        ResponseOutparam::set(response_out, Ok(response));
    }
}
```

### Example 3: Building Component

```bash
# Rust component
cargo component build --release --target wasm32-wasip2

# The component will be at:
# target/wasm32-wasip2/release/<name>.wasm

# Inspect the component
wash inspect ./target/wasm32-wasip2/release/component.wasm

# Test locally
wash dev
```

### Example 4: Component with Logging

```rust
use wasi::logging::logging::*;

fn process_request() {
    log(
        Level::Info,
        "request-handler",
        "Processing incoming request"
    );

    // ... processing logic ...

    log(
        Level::Debug,
        "request-handler",
        &format!("Request processed in {}ms", elapsed)
    );
}
```

## Guidelines

### Best Practices

- **Keep components small and focused**: Single responsibility principle
- **Design interfaces first**: Write WIT files before implementation
- **Use proper error types**: Return `result<T, E>` from WIT functions
- **Avoid blocking operations**: Components should respond quickly
- **Use structured logging**: Include context in log messages
- **Version your interfaces**: Use semantic versioning for WIT packages
- **Document your exports**: Clear documentation for public interfaces

### Performance Optimization

- Minimize allocations in hot paths
- Use streaming interfaces for large data
- Batch operations when possible
- Cache frequently accessed data (via providers)
- Profile with `wash spy` to identify bottlenecks

### Security Considerations

- Never trust input data - always validate
- Use typed interfaces to prevent injection attacks
- Leverage the component model's sandboxing
- Don't leak sensitive data in error messages
- Use secrets providers for credentials, never hardcode

### Error Handling

```rust
// Good: Proper error propagation
fn process() -> Result<Response, Error> {
    let data = fetch_data()?;
    let processed = transform(data)?;
    Ok(create_response(processed))
}

// Bad: Unwrapping errors
fn process() -> Response {
    let data = fetch_data().unwrap(); // Can panic!
    create_response(data)
}
```

## Common Patterns

### Request/Response Pattern

Components typically follow a request/response pattern:
1. Receive request through exported interface
2. Validate and parse request data
3. Call imported capabilities (providers) as needed
4. Process business logic
5. Format and return response

### State Management Pattern

Since components are stateless:
1. Use key-value providers for persistent state
2. Pass state tokens/IDs in requests
3. Use immutable data structures
4. Let providers handle consistency

### Composition Pattern

For complex applications:
1. Break functionality into focused components
2. Define clear interface contracts (WIT)
3. Use wadm to orchestrate components
4. Share common types through WIT packages

## Debugging Tips

- Use `wash spy` to intercept and inspect messages
- Add extensive logging during development
- Test components in isolation first
- Use `wash call` to manually invoke component functions
- Check component metadata with `wash inspect`

## Common Issues

**Build fails with "target not found"**
- Ensure wasm32-wasip2 target is installed: `rustup target add wasm32-wasip2`

**Component won't load in wasmCloud**
- Verify it's a valid component: `wash inspect component.wasm`
- Check WIT interfaces match provider capabilities
- Ensure proper exports are defined

**Runtime errors with providers**
- Verify provider is linked to component in wadm manifest
- Check link configuration is correct
- Ensure provider supports the required interface version

## Resources

- Component Model Book: https://component-model.bytecodealliance.org
- WASI 0.2 API Docs: https://docs.rs/wasi/latest/wasi/
- wasmCloud Examples: https://github.com/wasmCloud/wasmCloud/tree/main/examples
- WIT Reference: https://component-model.bytecodealliance.org/design/wit.html
