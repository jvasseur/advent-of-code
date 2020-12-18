import { assertEquals } from 'https://deno.land/std@0.80.0/testing/asserts.ts';
import compute from './compute.ts';

Deno.test('Compute examples', () => {
    assertEquals(compute('1 + (2 * 3) + (4 * (5 + 6))'), 51);
    assertEquals(compute('2 * 3 + (4 * 5)'), 46);
    assertEquals(compute('5 + (8 * 3 + 9 + 3 * 4 * 3)'), 1445);
    assertEquals(compute('5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))'), 669060);
    assertEquals(compute('((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2'), 23340);
});
