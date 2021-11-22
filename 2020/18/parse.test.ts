import { assertEquals } from 'https://deno.land/std@0.80.0/testing/asserts.ts';
import parse from './parse.ts';

Deno.test('Parse without parentesis', () => {
    assertEquals(
        parse('1 + 2'),
        [1, '+', 2],
    );
    assertEquals(
        parse('1 + 2 * 3 + 4 * 5 + 6'),
        [1, '+', 2, '*', 3, '+', 4, '*', 5, '+', 6],
    );
});

Deno.test('Parse with parentesis', () => {
    assertEquals(
        parse('1 + (2 * 3) + (4 * (5 + 6))'),
        [1, '+', [2, '*', 3], '+', [4, '*', [5, '+', 6]]],
    );

    assertEquals(
        parse('5 + (8 * 3 + 9 + 3 * 4 * 3)'),
        [5, '+', [8, '*', 3, '+', 9, '+', 3, '*', 4, '*', 3]],
    );
});
