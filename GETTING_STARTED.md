# Getting Started with WasmCloud Skills

This guide will help you start using the WasmCloud skills for AI-assisted development.

## What Are These Skills?

The WasmCloud skills in this repository are designed to help AI coding agents (like Claude Code) build distributed WebAssembly applications. Each skill provides expertise in a specific area of WasmCloud development.

## Skills Overview

### 1. **wasmcloud-setup** - Project Setup
Helps set up new WasmCloud projects with proper tooling and dependencies.

**When to use:** Starting a new WasmCloud project, verifying environment setup.

**Example prompts:**
- "Set up a new WasmCloud project for a microservice"
- "Verify my WasmCloud development environment"
- "Initialize a Rust component project"

### 2. **wasmcloud-component** - Component Development
Guides building WebAssembly components with WASI-P2.

**When to use:** Implementing component logic, working with WASI interfaces.

**Example prompts:**
- "Create an HTTP handler component"
- "Add key-value storage to my component"
- "Implement error handling in my component"

### 3. **wasmcloud-wit** - Interface Design
Helps design WIT (WebAssembly Interface Types) definitions.

**When to use:** Defining component interfaces, creating type definitions.

**Example prompts:**
- "Design WIT interfaces for a user service"
- "Create types for my data model in WIT"
- "Define a world for my component"

### 4. **wasmcloud-wadm** - Application Deployment
Assists with creating wadm deployment manifests.

**When to use:** Deploying applications, configuring multi-component systems.

**Example prompts:**
- "Create a wadm manifest for my application"
- "Configure scaling for my components"
- "Set up multi-region deployment"

### 5. **wasmcloud-deploy** - Deployment Management
Guides deploying and managing components in wasmCloud.

**When to use:** Deploying to environments, managing running applications.

**Example prompts:**
- "Deploy my component to wasmCloud"
- "Update a running application"
- "Roll back to a previous version"

### 6. **wasmcloud-provider** - Working with Providers
Helps work with capability providers.

**When to use:** Configuring providers, linking components to capabilities.

**Example prompts:**
- "Set up HTTP server provider"
- "Configure Redis for key-value storage"
- "Link my component to a database"

### 7. **wasmcloud-dev** - Development Workflow
Provides guidance on development workflows and best practices.

**When to use:** Setting up dev environment, debugging, testing.

**Example prompts:**
- "Set up hot-reload development"
- "Debug my component"
- "Create integration tests"

## How to Use Skills

### With Claude Code

Skills can be loaded automatically when you ask questions related to their domain. Simply describe what you want to build, and Claude will use the appropriate skills.

**Example conversation:**
```
User: I want to build a REST API in WasmCloud that stores data in Redis

Claude: [Loads wasmcloud-setup, wasmcloud-component, wasmcloud-provider skills]
        Let me help you build a REST API with Redis storage...
```

### Loading Skills Manually

If using a skills framework, you can load skills explicitly:

```
/skill wasmcloud-setup
/skill wasmcloud-component
```

## Demo: Recipe Management System

The repository includes a comprehensive demo - an AI-powered recipe management system built with WasmCloud.

### Features
- RESTful API for recipe CRUD operations
- Redis-backed persistent storage
- WebAssembly components with WASI-P2
- Distributed architecture

### Try the Demo

1. **Prerequisites:**
   ```bash
   # Install Rust and WasmCloud tools
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   cargo install wash-cli cargo-component --locked
   rustup target add wasm32-wasip2
   ```

2. **Start Redis:**
   ```bash
   docker run -d -p 6379:6379 redis:latest
   ```

3. **Build and Deploy:**
   ```bash
   cd demo/recipe-system
   ./scripts/build.sh
   ./scripts/deploy.sh
   ```

4. **Test:**
   ```bash
   ./scripts/test.sh
   ```

### Demo Architecture

```
User Requests
     ↓
Recipe API (WASM Component)
     ↓
  ┌──┴──┐
  ↓     ↓
HTTP  Key-Value
Server Provider
        ↓
      Redis
```

### API Examples

**Create a Recipe:**
```bash
curl -X POST http://localhost:8080/api/recipes \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Pancakes",
    "ingredients": [...],
    "instructions": [...],
    ...
  }'
```

**List Recipes:**
```bash
curl http://localhost:8080/api/recipes
```

**Get Recipe:**
```bash
curl http://localhost:8080/api/recipes/{id}
```

## Common Workflows

### Creating a New Component

1. **Setup:** Use `wasmcloud-setup` skill
2. **Define Interfaces:** Use `wasmcloud-wit` skill
3. **Implement:** Use `wasmcloud-component` skill
4. **Deploy:** Use `wasmcloud-deploy` skill

### Adding a New Feature

1. **Update WIT:** Modify interface definitions
2. **Implement:** Add new functionality
3. **Test:** Use `wash dev` for hot-reload
4. **Deploy:** Update and redeploy

### Debugging Issues

1. **Check Logs:** `wash logs <component-id>`
2. **Inspect Traffic:** `wash spy`
3. **Verify Links:** `wash get links`
4. **Check Status:** `wadm app get <name>`

## Best Practices

1. **Start Small:** Begin with a simple component
2. **Define Interfaces First:** Design WIT files before implementation
3. **Test Locally:** Use `wash dev` for rapid iteration
4. **Use Providers:** Leverage existing providers instead of building custom ones
5. **Monitor:** Check logs and status regularly
6. **Document:** Keep README and comments up to date

## Troubleshooting

### Build Issues
- **Problem:** Component won't build
- **Solution:** Verify wasm32-wasip2 target is installed: `rustup target add wasm32-wasip2`

### Deployment Issues
- **Problem:** Deployment fails
- **Solution:** Check WasmCloud is running: `wash get hosts`

### Runtime Issues
- **Problem:** Component crashes
- **Solution:** Check logs: `wash logs <component-id>`

## Learning Resources

### Official Documentation
- [WasmCloud Docs](https://wasmcloud.com/docs)
- [Component Model](https://component-model.bytecodealliance.org)
- [WASI 0.2 Spec](https://github.com/WebAssembly/WASI/tree/main/wasip2)

### Examples
- [WasmCloud Examples](https://github.com/wasmCloud/wasmCloud/tree/main/examples)
- Recipe System Demo (this repo)

### Community
- [WasmCloud Discord](https://discord.gg/wasmcloud)
- [Component Model Zulip](https://bytecodealliance.zulipchat.com)

## Next Steps

1. **Explore Skills:** Read through each skill's SKILL.md file
2. **Try the Demo:** Build and run the recipe management system
3. **Build Your Own:** Start with a simple component using the skills
4. **Contribute:** Add new skills or improve existing ones

## Support

If you encounter issues or have questions:

1. Check the troubleshooting section above
2. Review skill documentation in `skills/*/SKILL.md`
3. Look at the demo implementation
4. Open an issue on GitHub

## Contributing

Contributions welcome! Please:
1. Follow the existing skill pattern
2. Include comprehensive examples
3. Test with Claude Code
4. Update documentation

---

Happy building with WasmCloud! 🚀
