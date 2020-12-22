import { assertEquals } from 'https://deno.land/std@0.80.0/testing/asserts.ts';
import game from './game.ts';

Deno.test('Example', () => {
    assertEquals(
        game(
            [9, 2, 6, 3, 1],
            [5, 8, 4, 7, 10],
        ),
        [
            1,
            [],
            [7, 5, 6, 2, 4, 1, 10, 8, 9, 3],
        ],
    );
});

Deno.test('Recursive', () => {
    assertEquals(
        game(
            [43, 19],
            [2, 29, 14],
        ),
        [
            0,
            [43, 19],
            [2, 29, 14],
        ],
    );
})
