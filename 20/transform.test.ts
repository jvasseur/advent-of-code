import { assertEquals, assertNotStrictEquals, assertStrictEquals } from 'https://deno.land/std@0.80.0/testing/asserts.ts';
import transform from './transform.ts';
import { BaseTile } from './Tile.ts';

Deno.test('Transform', () => {
    const tile = new BaseTile(0, [
        ['#', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
        ['.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
        ['.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
        ['.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
        ['.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
        ['.', '.', '.', '.', '.', '.', '.', '.', '.', '#'],
        ['.', '.', '.', '.', '.', '.', '.', '.', '.', '#'],
        ['.', '.', '.', '.', '.', '.', '.', '#', '.', '.'],
        ['.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
        ['.', '.', '.', '.', '#', '.', '.', '.', '.', '.'],
    ]);

    const transformations = transform(tile);

    assertEquals(transformations.length, 12);

    assertStrictEquals(transformations[0], tile);
    assertNotStrictEquals(transformations[1], tile);
});
