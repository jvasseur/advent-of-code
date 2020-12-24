import { assertEquals } from 'https://deno.land/std@0.80.0/testing/asserts.ts';
import canonicalize from './canonicalize.ts';
import { Direction } from './types.ts';

Deno.test('canonicalize', () => {
    assertEquals(canonicalize([Direction.E, Direction.SE, Direction.NE, Direction.E]), { e: 3, ne: 0 });
    assertEquals(canonicalize([Direction.NW, Direction.W, Direction.SW, Direction.E, Direction.E]), { e: 0, ne: 0 });
});
