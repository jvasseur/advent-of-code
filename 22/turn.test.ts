import { assertEquals } from 'https://deno.land/std@0.80.0/testing/asserts.ts';
import turn from './turn.ts';

Deno.test('Round 1', () => {
    assertEquals(
        turn(
            [9, 2, 6, 3, 1],
            [5, 8, 4, 7, 10],
        ),
        [
            [2, 6, 3, 1, 9, 5],
            [8, 4, 7, 10],
        ]
    )
});

Deno.test('Round 2', () => {
    assertEquals(
        turn(
            [2, 6, 3, 1, 9, 5],
            [8, 4, 7, 10],
        ),
        [
            [6, 3, 1, 9, 5],
            [4, 7, 10, 8, 2],
        ]
    )
});

Deno.test('Round 27', () => {
    assertEquals(
        turn(
            [5, 4, 1],
            [8, 9, 7, 3, 2, 10, 6],
        ),
        [
            [4, 1],
            [9, 7, 3, 2, 10, 6, 8, 5],
        ]
    )
});
