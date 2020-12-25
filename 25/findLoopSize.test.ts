import { assertEquals } from 'https://deno.land/std@0.80.0/testing/asserts.ts';
import findLoopSize from './findLoopSize.ts';

Deno.test('findLoopSize', () => {
    assertEquals(findLoopSize(5764801), 8);
    assertEquals(findLoopSize(17807724), 11);
});
