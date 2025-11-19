# WasmCloud Skills for AI-Assisted Development

A comprehensive set of skills for AI coding agents (like Claude Code) to build distributed WebAssembly applications using WasmCloud and WASI-P2.

## Overview

This repository contains:
1. **Seven specialized skills** for WasmCloud development
2. **A comprehensive demo**: AI-powered recipe management system
3. **Documentation** and best practices

## Skills

The following skills help AI coding agents build, deploy, and manage WasmCloud applications:

### 1. **wasmcloud-setup**
Set up WasmCloud project environments with proper tooling and dependencies.
- Verify and install prerequisites (wash, Rust, wasm32-wasip2)
- Initialize projects with appropriate templates
- Configure development environment
- Set up project structure

### 2. **wasmcloud-component**
Build WebAssembly components with WASI-P2 for WasmCloud deployment.
- Component architecture patterns
- WASI 0.2 interface usage
- Error handling and best practices
- Performance optimization
- Security considerations

### 3. **wasmcloud-wit**
Design and implement WIT (WebAssembly Interface Types) definitions.
- WIT syntax and structure
- Interface and world definitions
- Type system (records, variants, enums, resources)
- Package management
- Code generation integration

### 4. **wasmcloud-wadm**
Create and manage wadm manifests for application deployment.
- Application manifest structure
- Component and provider configuration
- Scaling strategies (spreadscaler, daemonscaler)
- Linking components and providers
- Multi-environment deployments

### 5. **wasmcloud-deploy**
Deploy and manage components in wasmCloud environments.
- Host management
- Deployment methods (wadm, wash dev, direct)
- Provider deployment
- Link management
- CI/CD integration

### 6. **wasmcloud-provider**
Work with capability providers for external resource access.
- Standard WASI providers (HTTP, Key-Value, Messaging, Blob)
- Provider lifecycle management
- Linking strategies
- Configuration and secrets
- Custom provider development

### 7. **wasmcloud-dev**
Development workflow and best practices.
- Rapid iteration with `wash dev`
- Testing strategies (unit, integration, contract)
- Debugging tools (wash spy, wash call)
- Performance profiling
- CI/CD workflows

## Demo Application: Recipe Management System

An AI-powered recipe management system showcasing WasmCloud's capabilities:

### Features
- **Recipe CRUD**: Create, read, update, delete recipes
- **AI Chat Interface**: Chat with recipes to modify them
- **Cost Calculations**: Estimate ingredient costs
- **Recipe Remixing**: AI-powered recipe variations
- **Diet Adaptation**: Adapt recipes to dietary restrictions (vegan, keto, etc.)
- **Ingredient Substitution**: Suggest substitutes for unavailable ingredients
- **Nutritional Information**: Calculate nutrition facts
- **Recipe Search**: Search by ingredients, tags, dietary info

### Architecture
```
┌─────────────────────┐
│   HTTP Requests     │
└──────────┬──────────┘
           │
┌──────────▼──────────┐
│  Recipe API         │◄──────┐
│  (HTTP Handler)     │       │
└──────────┬──────────┘       │
           │                  │
    ┌──────┴──────┐          │
    │             │          │
┌───▼────┐   ┌───▼────┐    │
│ Recipe │   │ Recipe │    │
│  Chat  │   │  Calc  │    │
│  (AI)  │   │        │    │
└───┬────┘   └───┬────┘    │
    │            │         │
    └──────┬─────┘         │
           │               │
    ┌──────▼──────┐        │
    │  Key-Value  │────────┘
    │  Provider   │
    │  (SQLite)   │
    └─────────────┘
```

### Components
1. **recipe-api**: HTTP API gateway handling requests
2. **recipe-chat**: AI-powered chat interface using Claude API
3. **recipe-calculator**: Cost and nutrition calculations
4. **sqlite-provider**: SQLite-backed key-value storage

## Getting Started

### Prerequisites
```bash
# Rust toolchain
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# WasmCloud tools
cargo install wash-cli cargo-component wkg --locked
rustup target add wasm32-wasip2

# Verify installation
wash --version
cargo component --version
```

### Quick Start
```bash
# Clone repository
git clone <repo-url>
cd wasmcloud_skills

# Start WasmCloud
wash up

# Build and run demo
cd demo/recipe-system
./build.sh
./deploy.sh

# Access application
curl http://localhost:8080/api/recipes
```

## Using Skills with Claude Code

### Loading Skills
Skills can be loaded in Claude Code conversations using the `/skill` command or by referencing the skill directory.

### Example Workflow
```
User: "I need to create a new WasmCloud component for user authentication"

Claude: *loads wasmcloud-setup and wasmcloud-component skills*
         "Let me help you set up an authentication component..."

User: "Now create a wadm manifest to deploy it"

Claude: *loads wasmcloud-wadm skill*
         "I'll create a deployment manifest..."
```

## Project Structure
```
wasmcloud_skills/
├── README.md                          # This file
├── skills/                            # AI skills directory
│   ├── wasmcloud-setup/
│   ├── wasmcloud-component/
│   ├── wasmcloud-wit/
│   ├── wasmcloud-wadm/
│   ├── wasmcloud-deploy/
│   ├── wasmcloud-provider/
│   └── wasmcloud-dev/
├── demo/                              # Recipe management demo
│   └── recipe-system/
│       ├── components/                # WASM components
│       │   ├── api/
│       │   ├── chat/
│       │   └── calculator/
│       ├── providers/                 # Custom providers
│       │   └── sqlite/
│       ├── wit/                       # Shared WIT definitions
│       ├── wadm.yaml                  # Deployment manifest
│       ├── build.sh                   # Build script
│       └── deploy.sh                  # Deployment script
└── docs/                              # Additional documentation
    ├── ARCHITECTURE.md
    ├── DEPLOYMENT.md
    └── DEVELOPMENT.md
```

## Environment Variables

For the demo application:
```bash
# Claude API key for AI features
export CLAUDE_API_KEY="your-api-key-here"

# Optional: Custom NATS URL
export NATS_URL="nats://localhost:4222"

# Optional: Database path
export DB_PATH="./data/recipes.db"
```

## Testing

```bash
# Unit tests
cargo test

# Integration tests
./scripts/integration-test.sh

# End-to-end tests
./scripts/e2e-test.sh
```

## Development Workflow

1. **Design interfaces** (WIT files)
2. **Implement components** (Rust code)
3. **Test locally** (`wash dev`)
4. **Create deployment manifest** (wadm.yaml)
5. **Deploy** (`wadm app deploy`)
6. **Monitor** (`wash get inventory`)

## Resources

### WasmCloud
- [Official Documentation](https://wasmcloud.com/docs)
- [GitHub Repository](https://github.com/wasmCloud/wasmCloud)
- [Community Discord](https://discord.gg/wasmcloud)

### WebAssembly Component Model
- [Component Model Docs](https://component-model.bytecodealliance.org)
- [WIT Specification](https://component-model.bytecodealliance.org/design/wit.html)
- [WASI 0.2 Preview](https://github.com/WebAssembly/WASI/tree/main/wasip2)

### Skills Framework
- [Anthropic Skills Repository](https://github.com/anthropics/skills)
- [Claude Code Documentation](https://docs.claude.com/claude-code)

## Contributing

Contributions welcome! Please:
1. Follow existing skill patterns
2. Include comprehensive examples
3. Test skills with Claude Code
4. Update documentation

## License

MIT License - see LICENSE file for details

## Acknowledgments

- wasmCloud team for the excellent platform
- Anthropic for Claude and skills framework
- Bytecode Alliance for WebAssembly Component Model
