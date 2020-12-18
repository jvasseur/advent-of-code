import { assertEquals } from 'https://deno.land/std@0.80.0/testing/asserts.ts';
import compute from './compute.ts';

Deno.test('Compute without parentesis', () => {
    assertEquals(compute('1 + 2 * 3 + 4 * 5 + 6'), 71);
});

Deno.test('Compute with parentesis', () => {
    assertEquals(compute('1 + (2 * 3) + (4 * (5 + 6))'), 51);
});

Deno.test('Compute other examples', () => {
    assertEquals(compute('2 * 3 + (4 * 5)'), 26);
    assertEquals(compute('5 + (8 * 3 + 9 + 3 * 4 * 3)'), 437);
    assertEquals(compute('5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))'), 12240);
    assertEquals(compute('((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2'), 13632);
});
