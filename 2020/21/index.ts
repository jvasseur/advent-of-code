import reduceAll from '../utils/reduceAll.ts';
import intersect from '../utils/intersect.ts';
import sort from '../utils/sort.ts';

interface Food {
    ingredients: string[];
    allergens: string[]
}

const input = await Deno.readTextFile(new URL('input.txt', import.meta.url));

const lines = input.replace(/\n$/, '').split('\n')

const foods: Food[] = lines.map((line) => {
    const match = /^(.*) \(contains (.*)\)$/.exec(line);

    if (!match) {
        throw new Error('Invalid line');
    }

    const [, ingredients, allergens] = match;

    return {
        ingredients: ingredients.split(' '),
        allergens: allergens.split(', '),
    };
});

const foodsByAllergen: Record<string, Food[]> = {};

for (const food of foods) {
    for (const allergen of food.allergens) {
        if (!(allergen in foodsByAllergen)) {
            foodsByAllergen[allergen] = [];
        }

        foodsByAllergen[allergen].push(food);
    }
}

const ingredientsByAllergen: Record<string, string[]> = Object.fromEntries(Object.entries(foodsByAllergen).map(([allergen, foods]) => {
    const ingredients = reduceAll(intersect, foods.map((food) => food.ingredients));

    return [allergen, ingredients]
}));

const allergenIngredient: Record<string, string> = {};

while (Object.keys(allergenIngredient).length < Object.keys(ingredientsByAllergen).length) {
    for (const [allergen, ingredients] of Object.entries(ingredientsByAllergen)) {
        const unknownIngredients = ingredients.filter((ingredient) => !Object.values(allergenIngredient).includes(ingredient));

        if (unknownIngredients.length === 1) {
            allergenIngredient[allergen] = unknownIngredients[0];
        }
    }
}

let safeIngredientCount = 0;

for (const food of foods) {
    for (const ingredient of food.ingredients) {
        if (!Object.values(allergenIngredient).includes(ingredient)) {
            safeIngredientCount++;
        }
    }
}

console.log(safeIngredientCount);

console.log(sort(Object.entries(allergenIngredient), ([a], [b]) => a.localeCompare(b)).map(([, ingredient]) => ingredient).join(','));
