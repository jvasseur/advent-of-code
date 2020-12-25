import { assertEquals } from 'https://deno.land/std@0.80.0/testing/asserts.ts';
import transform from './transform.ts';

Deno.test('transform', () => {
    assertEquals(transform(17807724, 8), 14897079);
    assertEquals(transform(5764801, 11), 14897079);
});
