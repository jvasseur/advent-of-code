import { assertEquals } from 'https://deno.land/std@0.80.0/testing/asserts.ts';
import applyMask from './applyMask.ts';

Deno.test('example', () => {
    assertEquals(applyMask(11, 'XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X'), 73);
    assertEquals(applyMask(101, 'XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X'), 101);
    assertEquals(applyMask(0, 'XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X'), 64);
});
