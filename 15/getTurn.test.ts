import { assertEquals } from 'https://deno.land/std@0.80.0/testing/asserts.ts';
import getTurn from './getTurn.ts';

Deno.test('It computes the correct iterations', () => {
    assertEquals(getTurn([0, 3, 6], 4), 0);
    assertEquals(getTurn([0, 3, 6], 5), 3);
    assertEquals(getTurn([0, 3, 6], 6), 3);
    assertEquals(getTurn([0, 3, 6], 7), 1);
    assertEquals(getTurn([0, 3, 6], 8), 0);
    assertEquals(getTurn([0, 3, 6], 9), 4);
    assertEquals(getTurn([0, 3, 6], 10), 0);
});


Deno.test('It returns the correct results', () => {
    assertEquals(getTurn([1, 3, 2], 2020), 1);
    assertEquals(getTurn([2, 1, 3], 2020), 10);
    assertEquals(getTurn([1, 2, 3], 2020), 27);
    assertEquals(getTurn([2, 3, 1], 2020), 78);
    assertEquals(getTurn([3, 2, 1], 2020), 438);
    assertEquals(getTurn([3, 1, 2], 2020), 1836);
});
