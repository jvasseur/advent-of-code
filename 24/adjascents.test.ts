import { assertEquals } from 'https://deno.land/std@0.80.0/testing/asserts.ts';
import { getAdjascents } from './adjascents.ts';
import { Canonical } from './types.ts';

Deno.test('getAdjascents', () => {
    assertEquals(getAdjascents({ e: 0, ne: 0 }), [
        { e: 1, ne: 0 },
        { e: 0, ne: 1 },
        { e: -1, ne: 1 },
        { e: -1, ne: 0 },
        { e: 0, ne: -1 },
        { e: 1, ne: -1 },
    ]);
});
