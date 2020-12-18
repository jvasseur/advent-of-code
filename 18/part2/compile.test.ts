import { assertEquals } from 'https://deno.land/std@0.80.0/testing/asserts.ts';
import compile from './compile.ts';

Deno.test('Compile examples', () => {
    // 2 * 3 + (4 * 5)
    assertEquals(
        compile([2, '*', 3, '+', [4, '*', 5]]),
        [2, '*', [3, '+', [4, '*', 5]]],
    );
    // 5 + (8 * 3 + 9 + 3 * 4 * 3)
    assertEquals(
        compile([5, '+', [8, '*', 3, '+', 9, '+', 3, '*', 4, '*', 3]]),
        [5, '+', [[[8, '*', [[3, '+', 9], '+', 3]], '*', 4], '*', 3]],
    );
});
