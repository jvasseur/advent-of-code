import { assertEquals } from 'https://deno.land/std@0.80.0/testing/asserts.ts';
import tokenize, { End } from './tokenize.ts';

Deno.test('Tokenize', () => {
    assertEquals(
        Array.from(tokenize('1 + 2')).map(({ value }) => value),
        [1, '+', 2, End],
    );
    assertEquals(
        Array.from(tokenize('1 + 2 * 3 + 4 * 5 + 6')).map(({ value }) => value),
        [1, '+', 2, '*', 3, '+', 4, '*', 5, '+', 6, End],
    );
    assertEquals(
        Array.from(tokenize('5 + (8 * 3 + 9 + 3 * 4 * 3)')).map(({ value }) => value),
        [5, '+', '(', 8, '*', 3, '+', 9, '+', 3, '*', 4, '*', 3, ')', End],
    );
});
