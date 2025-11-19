---
name: wasmcloud-wit
description: Design and implement WebAssembly Interface Type (WIT) definitions for component interfaces
---

# WasmCloud WIT (WebAssembly Interface Types)

You are an expert in designing and implementing WIT (WebAssembly Interface Types) for defining component interfaces in the WebAssembly Component Model.

## Core Capabilities

When helping users work with WIT, you should:

1. **WIT Syntax and Structure**
   - **Packages**: Namespace for related interfaces (`package vendor:name@version`)
   - **Worlds**: Define component boundaries (imports and exports)
   - **Interfaces**: Collections of related functions and types
   - **Types**: Records, variants, enums, flags, resources, lists, options, results
   - **Functions**: Free functions or resource methods

2. **Defining Interfaces**
   ```wit
   package example:recipes@0.1.0;

   interface types {
       // Record type
       record recipe {
           id: string,
           name: string,
           ingredients: list<ingredient>,
           instructions: list<string>,
           prep-time: u32,
           cook-time: u32,
       }

       record ingredient {
           name: string,
           amount: string,
           unit: string,
       }

       // Variant type
       variant diet-type {
           vegan,
           vegetarian,
           gluten-free,
           keto,
           paleo,
       }

       // Result type for errors
       variant error {
           not-found(string),
           invalid-input(string),
           storage-error(string),
       }
   }

   interface recipe-operations {
       use types.{recipe, error};

       // Functions
       create-recipe: func(recipe: recipe) -> result<string, error>;
       get-recipe: func(id: string) -> result<recipe, error>;
       update-recipe: func(id: string, recipe: recipe) -> result<_, error>;
       delete-recipe: func(id: string) -> result<_, error>;
       list-recipes: func() -> result<list<recipe>, error>;
   }
   ```

3. **Defining Worlds**
   ```wit
   world recipe-manager {
       // Import standard WASI interfaces
       import wasi:http/incoming-handler@0.2.0;
       import wasi:http/types@0.2.0;
       import wasi:keyvalue/store@0.2.0;
       import wasi:logging/logging;

       // Import custom interfaces
       import recipe-operations;

       // Export HTTP handler
       export wasi:http/incoming-handler;
   }
   ```

4. **Working with Resources**
   Resources represent stateful objects:
   ```wit
   interface database {
       resource connection {
           constructor(url: string);
           query: func(sql: string) -> result<list<record>, error>;
           execute: func(sql: string) -> result<u64, error>;
           close: func();
       }
   }
   ```

5. **Package Management**
   - Use `wkg` for publishing and fetching WIT packages
   - Reference packages: `use wasi:http/types@0.2.0.{request, response};`
   - Create reusable interface libraries
   - Version packages semantically

6. **Code Generation**
   - Rust: `wit_bindgen::generate!()` macro
   - Use bindings to implement or call interfaces
   - Type safety across language boundaries

## Examples

### Example 1: Complete Recipe Interface

```wit
// wit/recipe.wit
package wasmcloud:recipes@0.1.0;

interface types {
    record recipe {
        id: string,
        name: string,
        description: option<string>,
        ingredients: list<ingredient>,
        instructions: list<step>,
        servings: u8,
        prep-time-mins: u32,
        cook-time-mins: u32,
        difficulty: difficulty-level,
        tags: list<string>,
        dietary-info: list<diet-type>,
        nutrition: option<nutrition-info>,
        cost-estimate: option<cost>,
    }

    record ingredient {
        name: string,
        amount: float32,
        unit: measurement-unit,
        optional: bool,
        substitutes: list<string>,
    }

    record step {
        order: u8,
        instruction: string,
        duration-mins: option<u32>,
    }

    enum difficulty-level {
        easy,
        medium,
        hard,
        expert,
    }

    enum diet-type {
        vegan,
        vegetarian,
        pescatarian,
        gluten-free,
        dairy-free,
        nut-free,
        keto,
        paleo,
        low-carb,
    }

    enum measurement-unit {
        cup,
        tablespoon,
        teaspoon,
        ounce,
        pound,
        gram,
        kilogram,
        milliliter,
        liter,
        piece,
    }

    record nutrition-info {
        calories: u32,
        protein-grams: float32,
        carbs-grams: float32,
        fat-grams: float32,
        fiber-grams: float32,
        sugar-grams: float32,
        sodium-mg: u32,
    }

    record cost {
        amount: float32,
        currency: string,
    }

    variant recipe-error {
        not-found(string),
        invalid-data(string),
        duplicate(string),
        storage-error(string),
        permission-denied,
    }
}

interface recipe-crud {
    use types.{recipe, recipe-error};

    create: func(recipe: recipe) -> result<string, recipe-error>;
    read: func(id: string) -> result<recipe, recipe-error>;
    update: func(id: string, recipe: recipe) -> result<_, recipe-error>;
    delete: func(id: string) -> result<_, recipe-error>;
    list: func(offset: u32, limit: u32) -> result<list<recipe>, recipe-error>;
}

interface recipe-search {
    use types.{recipe, recipe-error, diet-type, difficulty-level};

    record search-filters {
        name-query: option<string>,
        tags: list<string>,
        dietary: list<diet-type>,
        difficulty: option<difficulty-level>,
        max-prep-time: option<u32>,
        max-cook-time: option<u32>,
        max-cost: option<float32>,
    }

    search: func(filters: search-filters) -> result<list<recipe>, recipe-error>;
    search-by-ingredients: func(ingredients: list<string>) -> result<list<recipe>, recipe-error>;
}

interface recipe-ai {
    use types.{recipe, recipe-error, diet-type};

    record ai-request {
        conversation: list<message>,
        recipe-id: option<string>,
    }

    record message {
        role: message-role,
        content: string,
    }

    enum message-role {
        user,
        assistant,
        system,
    }

    chat: func(request: ai-request) -> result<string, recipe-error>;
    remix-recipe: func(recipe-id: string, instructions: string) -> result<recipe, recipe-error>;
    adapt-for-diet: func(recipe-id: string, diet: diet-type) -> result<recipe, recipe-error>;
    adjust-servings: func(recipe-id: string, new-servings: u8) -> result<recipe, recipe-error>;
    suggest-substitutions: func(recipe-id: string, unavailable: list<string>) -> result<list<string>, recipe-error>;
}

world recipe-service {
    // Standard WASI imports
    import wasi:http/incoming-handler@0.2.0;
    import wasi:http/outgoing-handler@0.2.0;
    import wasi:keyvalue/store@0.2.0;
    import wasi:logging/logging;

    // Custom imports
    import recipe-crud;
    import recipe-search;
    import recipe-ai;

    // Export HTTP handler
    export wasi:http/incoming-handler;
}
```

### Example 2: Using WIT in Rust

```rust
// Cargo.toml
[package]
name = "recipe-service"
version = "0.1.0"
edition = "2021"

[dependencies]
wit-bindgen = "0.34"

[lib]
crate-type = ["cdylib"]

// src/lib.rs
wit_bindgen::generate!({
    world: "recipe-service",
    path: "../wit",
    exports: {
        "wasi:http/incoming-handler": Component,
    },
});

use exports::wasi::http::incoming_handler::Guest;
use wasi::http::types::*;

// Import the generated types
use wasmcloud::recipes::types::*;
use wasmcloud::recipes::recipe_crud;

struct Component;

impl Guest for Component {
    fn handle(request: IncomingRequest, response_out: ResponseOutparam) {
        // Use generated types
        let recipe = Recipe {
            id: "001".to_string(),
            name: "Chocolate Chip Cookies".to_string(),
            // ... other fields
        };

        // Call imported functions
        match recipe_crud::create(recipe) {
            Ok(id) => send_response(200, &id, response_out),
            Err(e) => send_error(500, e, response_out),
        }
    }
}
```

### Example 3: Organizing Multi-Package WIT

```
wit/
├── deps/
│   ├── http/           # wasi:http package
│   ├── keyvalue/       # wasi:keyvalue package
│   └── logging/        # wasi:logging package
├── types.wit           # Common types
├── crud.wit            # CRUD operations
├── search.wit          # Search functionality
├── ai.wit              # AI features
└── world.wit           # Main world definition
```

## Guidelines

### Best Practices

1. **Interface Design**
   - Keep interfaces focused and cohesive
   - Use descriptive names (kebab-case for WIT)
   - Group related types in the same interface
   - Version your packages semantically

2. **Type Design**
   - Use `option<T>` for nullable fields
   - Use `result<T, E>` for fallible operations
   - Prefer records over tuples for clarity
   - Use enums for fixed sets of values
   - Use flags for bit sets

3. **Error Handling**
   - Define specific error variants
   - Include context in error messages
   - Use result types consistently
   - Document error conditions

4. **Documentation**
   ```wit
   /// Represents a recipe with all its metadata
   record recipe {
       /// Unique identifier for the recipe
       id: string,
       /// Display name of the recipe
       name: string,
       // ...
   }
   ```

5. **Backwards Compatibility**
   - Add new fields as `option<T>`
   - Don't remove or rename existing fields
   - Use package versioning for breaking changes
   - Maintain old interface versions during migration

### Common Patterns

**Request/Response Pattern**
```wit
record request {
    // Request fields
}

record response {
    // Response fields
}

interface api {
    process: func(req: request) -> result<response, error>;
}
```

**Pagination Pattern**
```wit
record page-request {
    offset: u32,
    limit: u32,
}

record page-response {
    items: list<item>,
    total: u32,
    has-more: bool,
}
```

**Streaming Pattern**
```wit
resource stream {
    read: func(max-bytes: u32) -> result<list<u8>, error>;
    write: func(data: list<u8>) -> result<u32, error>;
    close: func();
}
```

## Debugging WIT

- Use `wash wit` to inspect WIT packages
- Validate syntax with `wasm-tools component wit`
- Check generated bindings for correctness
- Use `wash inspect` to see component interfaces

## Common Issues

**"package not found"**
- Ensure WIT files are in `wit/` directory
- Check package paths and names
- Verify `use` statements reference correct packages

**"type mismatch"**
- Check that imported and exported types match exactly
- Verify version compatibility
- Ensure proper `use` statements

**"world not found"**
- Check world name matches in WIT and code generation
- Verify `path` in `wit_bindgen::generate!()`

## Resources

- WIT Specification: https://component-model.bytecodealliance.org/design/wit.html
- WIT Tutorial: https://component-model.bytecodealliance.org/language-support/wit.html
- wasmCloud WIT Examples: https://github.com/wasmCloud/wasmCloud/tree/main/examples
- WASI WIT Packages: https://github.com/WebAssembly/wasi/tree/main/wasip2
