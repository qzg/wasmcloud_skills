# Recipe Management System

An AI-powered recipe management system built with WasmCloud and WASI-P2, demonstrating distributed WebAssembly applications.

## Features

- **Recipe CRUD Operations**: Create, read, update, and delete recipes
- **RESTful API**: Standard HTTP API for recipe management
- **Persistent Storage**: Redis-backed key-value storage
- **WebAssembly Components**: Built using WASI 0.2 (WASI-P2)
- **Distributed Architecture**: Deployable across edge and cloud

## Architecture

```
┌─────────────────────┐
│   HTTP Requests     │
└──────────┬──────────┘
           │
┌──────────▼──────────┐
│  Recipe API         │
│  (WASM Component)   │
└──────────┬──────────┘
           │
    ┌──────┴──────┐
    │             │
┌───▼────┐   ┌───▼────┐
│  HTTP  │   │  KV    │
│ Server │   │ Store  │
│Provider│   │Provider│
└────────┘   └────────┘
                │
         ┌──────▼──────┐
         │    Redis    │
         └─────────────┘
```

## Prerequisites

- Rust toolchain
- WasmCloud tools (wash, cargo-component)
- Redis (for storage)
- wasm32-wasip2 target

### Installation

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install WasmCloud tools
cargo install wash-cli cargo-component --locked
rustup target add wasm32-wasip2

# Verify installation
wash --version
cargo component --version
```

## Quick Start

### 1. Start Redis

```bash
# Using Docker
docker run -d -p 6379:6379 redis:latest

# Or install Redis locally and start it
redis-server
```

### 2. Build the System

```bash
cd demo/recipe-system
./scripts/build.sh
```

### 3. Deploy to WasmCloud

```bash
# Start WasmCloud (if not running)
wash up

# Deploy the application
./scripts/deploy.sh
```

### 4. Test the API

```bash
# Run automated tests
./scripts/test.sh

# Or test manually
curl http://localhost:8080/health
curl http://localhost:8080/api/recipes
```

## API Endpoints

### Health Check
```bash
GET /health
```

### List All Recipes
```bash
GET /api/recipes
```

### Get Recipe by ID
```bash
GET /api/recipes/{id}
```

### Create Recipe
```bash
POST /api/recipes
Content-Type: application/json

{
  "id": "",
  "name": "Recipe Name",
  "description": "Recipe description",
  "ingredients": [
    {
      "name": "ingredient name",
      "amount": 1.0,
      "unit": "cup",
      "optional": false,
      "notes": null
    }
  ],
  "instructions": [
    {
      "order": 1,
      "instruction": "Step description",
      "duration_mins": 5
    }
  ],
  "servings": 4,
  "prep_time_mins": 15,
  "cook_time_mins": 30,
  "difficulty": "easy",
  "tags": ["tag1", "tag2"],
  "dietary_info": ["vegetarian"],
  "created_at": 0,
  "updated_at": 0
}
```

### Update Recipe
```bash
PUT /api/recipes/{id}
Content-Type: application/json

{
  "id": "{id}",
  "name": "Updated Name",
  ...
}
```

### Delete Recipe
```bash
DELETE /api/recipes/{id}
```

## Example Usage

### Create a Recipe

```bash
curl -X POST http://localhost:8080/api/recipes \
  -H "Content-Type: application/json" \
  -d '{
    "id": "",
    "name": "Pancakes",
    "description": "Fluffy pancakes",
    "ingredients": [
      {
        "name": "flour",
        "amount": 2,
        "unit": "cup",
        "optional": false,
        "notes": null
      },
      {
        "name": "milk",
        "amount": 1.5,
        "unit": "cup",
        "optional": false,
        "notes": null
      },
      {
        "name": "eggs",
        "amount": 2,
        "unit": "piece",
        "optional": false,
        "notes": null
      }
    ],
    "instructions": [
      {
        "order": 1,
        "instruction": "Mix ingredients",
        "duration_mins": 5
      },
      {
        "order": 2,
        "instruction": "Cook on griddle",
        "duration_mins": 10
      }
    ],
    "servings": 4,
    "prep_time_mins": 5,
    "cook_time_mins": 10,
    "difficulty": "easy",
    "tags": ["breakfast", "quick"],
    "dietary_info": ["vegetarian"],
    "created_at": 0,
    "updated_at": 0
  }'
```

### List All Recipes

```bash
curl http://localhost:8080/api/recipes | jq '.'
```

## Development

### Project Structure

```
recipe-system/
├── components/
│   └── api/
│       ├── Cargo.toml
│       ├── src/
│       │   └── lib.rs
│       └── wit/           # WIT interface definitions
├── wit/                   # Shared WIT definitions
│   ├── types.wit
│   ├── interfaces.wit
│   └── worlds.wit
├── wadm.yaml             # Deployment manifest
├── scripts/
│   ├── build.sh          # Build script
│   ├── deploy.sh         # Deployment script
│   └── test.sh           # Test script
└── README.md
```

### Building Components

```bash
cd components/api
cargo component build --release --target wasm32-wasip2
```

### Local Development with Hot Reload

```bash
cd components/api
wash dev
```

Changes to source files will automatically rebuild and redeploy the component.

## Deployment

### Development
```bash
wash up              # Start local WasmCloud
./scripts/deploy.sh  # Deploy the application
```

### Production

1. Push components to a registry:
```bash
wash push ghcr.io/myorg/recipe-api:v1.0.0 components/api/build/recipe_api_s.wasm
```

2. Update `wadm.yaml` with registry URLs

3. Deploy to production WasmCloud:
```bash
wadm app deploy wadm.yaml
```

## Monitoring

### Check Application Status
```bash
wadm app get recipe-system
```

### View Component Inventory
```bash
wash get inventory
```

### View Links
```bash
wash get links
```

### Check Logs
```bash
wash logs recipe-api
```

## Troubleshooting

### Component Won't Build
```bash
# Verify wasm32-wasip2 target is installed
rustup target add wasm32-wasip2

# Clean and rebuild
cargo clean
cargo component build --release --target wasm32-wasip2
```

### Deployment Fails
```bash
# Check WasmCloud is running
wash get hosts

# Verify Redis is running
redis-cli ping

# Check wadm status
wadm app get recipe-system
```

### API Not Responding
```bash
# Check if HTTP server provider is running
wash get inventory | grep http

# Verify component is running
wash get inventory | grep recipe-api

# Check links are established
wash get links
```

## Future Enhancements

- **AI Chat Interface**: Chat with recipes using Claude API
- **Cost Calculator**: Calculate ingredient costs
- **Recipe Remixing**: AI-powered recipe variations
- **Diet Adaptation**: Adapt recipes to dietary restrictions
- **Ingredient Substitution**: Suggest alternatives
- **Nutritional Calculator**: Calculate nutrition facts
- **Search & Filter**: Advanced recipe search

## License

MIT License

## Contributing

Contributions welcome! Please see the main repository README for guidelines.
