import { assertEquals } from 'https://deno.land/std@0.80.0/testing/asserts.ts';
import compile from './compile.ts';

Deno.test('Compile without parentesis', () => {
    assertEquals(
        compile([1, '+', 2, '*', 3, '+', 4, '*', 5, '+', 6]),
        [[[[[1, '+', 2], '*', 3], '+', 4], '*', 5], '+', 6],
    );
});

Deno.test('Compile with parentesis 1', () => {
    assertEquals(
        // 1 + (2 * 3) + (4 * (5 + 6))
        compile([1, '+', [2, '*', 3], '+', [4, '*', [5, '+', 6]]]),
        [[1, '+', [2, '*', 3]], '+', [4, '*', [5, '+', 6]]],
    );
});

Deno.test('Compile with parentesis 2', () => {
    assertEquals(
        // 5 + (8 * 3 + 9 + 3 * 4 * 3)
        compile([5, '+', [8, '*', 3, '+', 9, '+', 3, '*', 4, '*', 3]]),
        [5, '+', [[[[[8, '*', 3], '+', 9], '+', 3], '*', 4], '*', 3]],
    );
});
