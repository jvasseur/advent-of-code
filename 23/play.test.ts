import { assertEquals } from 'https://deno.land/std@0.80.0/testing/asserts.ts';
import play from './play.ts';

Deno.test('Example', () => {
    assertEquals(
        play([3, 8, 9, 1, 2, 5, 4, 6, 7], 10),
        [8, 3,  7,  4,  1,  9,  2,  6, 5],
    );
});
