#!/bin/bash

BASE_URL="http://localhost:8080"

echo "Testing Recipe Management System..."
echo ""

# Health check
echo "1. Health check..."
curl -s $BASE_URL/health | jq '.'
echo ""

# Create a recipe
echo "2. Creating a recipe..."
RECIPE1=$(cat <<'EOF'
{
  "id": "",
  "name": "Chocolate Chip Cookies",
  "description": "Classic homemade chocolate chip cookies",
  "ingredients": [
    {
      "name": "all-purpose flour",
      "amount": 2.25,
      "unit": "cup",
      "optional": false,
      "notes": null
    },
    {
      "name": "butter",
      "amount": 1,
      "unit": "cup",
      "optional": false,
      "notes": "softened"
    },
    {
      "name": "white sugar",
      "amount": 0.75,
      "unit": "cup",
      "optional": false,
      "notes": null
    },
    {
      "name": "brown sugar",
      "amount": 0.75,
      "unit": "cup",
      "optional": false,
      "notes": "packed"
    },
    {
      "name": "eggs",
      "amount": 2,
      "unit": "piece",
      "optional": false,
      "notes": null
    },
    {
      "name": "vanilla extract",
      "amount": 2,
      "unit": "teaspoon",
      "optional": false,
      "notes": null
    },
    {
      "name": "baking soda",
      "amount": 1,
      "unit": "teaspoon",
      "optional": false,
      "notes": null
    },
    {
      "name": "salt",
      "amount": 1,
      "unit": "teaspoon",
      "optional": false,
      "notes": null
    },
    {
      "name": "chocolate chips",
      "amount": 2,
      "unit": "cup",
      "optional": false,
      "notes": "semi-sweet"
    }
  ],
  "instructions": [
    {
      "order": 1,
      "instruction": "Preheat oven to 375°F (190°C).",
      "duration_mins": 5
    },
    {
      "order": 2,
      "instruction": "In a bowl, cream together butter and sugars until fluffy.",
      "duration_mins": 3
    },
    {
      "order": 3,
      "instruction": "Beat in eggs and vanilla extract.",
      "duration_mins": 2
    },
    {
      "order": 4,
      "instruction": "In separate bowl, combine flour, baking soda, and salt.",
      "duration_mins": 2
    },
    {
      "order": 5,
      "instruction": "Gradually blend dry ingredients into creamed mixture.",
      "duration_mins": 3
    },
    {
      "order": 6,
      "instruction": "Stir in chocolate chips.",
      "duration_mins": 1
    },
    {
      "order": 7,
      "instruction": "Drop rounded tablespoons of dough onto ungreased baking sheets.",
      "duration_mins": 5
    },
    {
      "order": 8,
      "instruction": "Bake for 9-11 minutes or until golden brown.",
      "duration_mins": 11
    }
  ],
  "servings": 48,
  "prep_time_mins": 20,
  "cook_time_mins": 11,
  "difficulty": "easy",
  "tags": ["dessert", "cookies", "chocolate", "baking"],
  "dietary_info": ["vegetarian"],
  "created_at": 0,
  "updated_at": 0
}
EOF
)

RESPONSE=$(curl -s -X POST -H "Content-Type: application/json" -d "$RECIPE1" $BASE_URL/api/recipes)
echo $RESPONSE | jq '.'
RECIPE_ID=$(echo $RESPONSE | jq -r '.id')
echo ""

# Get the recipe
echo "3. Getting recipe by ID ($RECIPE_ID)..."
curl -s $BASE_URL/api/recipes/$RECIPE_ID | jq '.'
echo ""

# List all recipes
echo "4. Listing all recipes..."
curl -s $BASE_URL/api/recipes | jq '.'
echo ""

# Create another recipe
echo "5. Creating another recipe (Guacamole)..."
RECIPE2=$(cat <<'EOF'
{
  "id": "",
  "name": "Classic Guacamole",
  "description": "Fresh and flavorful guacamole",
  "ingredients": [
    {
      "name": "avocados",
      "amount": 3,
      "unit": "piece",
      "optional": false,
      "notes": "ripe"
    },
    {
      "name": "lime",
      "amount": 1,
      "unit": "piece",
      "optional": false,
      "notes": "juiced"
    },
    {
      "name": "salt",
      "amount": 0.5,
      "unit": "teaspoon",
      "optional": false,
      "notes": null
    },
    {
      "name": "onion",
      "amount": 0.5,
      "unit": "cup",
      "optional": false,
      "notes": "diced"
    },
    {
      "name": "cilantro",
      "amount": 3,
      "unit": "tablespoon",
      "optional": false,
      "notes": "chopped"
    },
    {
      "name": "tomatoes",
      "amount": 2,
      "unit": "piece",
      "optional": true,
      "notes": "diced"
    },
    {
      "name": "garlic",
      "amount": 1,
      "unit": "teaspoon",
      "optional": true,
      "notes": "minced"
    }
  ],
  "instructions": [
    {
      "order": 1,
      "instruction": "Cut avocados in half, remove pit, and scoop out flesh.",
      "duration_mins": 3
    },
    {
      "order": 2,
      "instruction": "Mash avocados in a bowl with lime juice and salt.",
      "duration_mins": 2
    },
    {
      "order": 3,
      "instruction": "Mix in onion, cilantro, tomatoes, and garlic.",
      "duration_mins": 3
    },
    {
      "order": 4,
      "instruction": "Adjust seasoning to taste and serve immediately.",
      "duration_mins": 1
    }
  ],
  "servings": 6,
  "prep_time_mins": 10,
  "cook_time_mins": 0,
  "difficulty": "easy",
  "tags": ["appetizer", "mexican", "dip", "healthy"],
  "dietary_info": ["vegan", "vegetarian", "gluten-free", "dairy-free"],
  "created_at": 0,
  "updated_at": 0
}
EOF
)

curl -s -X POST -H "Content-Type: application/json" -d "$RECIPE2" $BASE_URL/api/recipes | jq '.'
echo ""

# List all recipes again
echo "6. Listing all recipes (should show 2)..."
curl -s $BASE_URL/api/recipes | jq '. | length'
echo ""

# Update a recipe
echo "7. Updating recipe..."
UPDATE=$(cat <<EOF
{
  "id": "$RECIPE_ID",
  "name": "Best Chocolate Chip Cookies Ever",
  "description": "The absolute best homemade chocolate chip cookies - updated!",
  "ingredients": [
    {
      "name": "all-purpose flour",
      "amount": 2.25,
      "unit": "cup",
      "optional": false,
      "notes": null
    },
    {
      "name": "butter",
      "amount": 1,
      "unit": "cup",
      "optional": false,
      "notes": "softened"
    }
  ],
  "instructions": [
    {
      "order": 1,
      "instruction": "Follow the updated recipe!",
      "duration_mins": 30
    }
  ],
  "servings": 48,
  "prep_time_mins": 20,
  "cook_time_mins": 11,
  "difficulty": "easy",
  "tags": ["dessert", "cookies", "chocolate"],
  "dietary_info": ["vegetarian"],
  "created_at": 0,
  "updated_at": 0
}
EOF
)

curl -s -X PUT -H "Content-Type: application/json" -d "$UPDATE" $BASE_URL/api/recipes/$RECIPE_ID | jq '.'
echo ""

# Verify update
echo "8. Verifying update..."
curl -s $BASE_URL/api/recipes/$RECIPE_ID | jq '.name'
echo ""

echo "All tests completed successfully!"
