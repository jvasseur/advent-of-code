import { assertEquals } from 'https://deno.land/std@0.80.0/testing/asserts.ts';
import parseLine from './parseLine.ts';

Deno.test('parseLine', () => {
    assertEquals(parseLine('esenee'), ['e', 'se', 'ne', 'e']);
    assertEquals(parseLine('esew'), ['e', 'se', 'w']);
    assertEquals(parseLine('nwwswee'), ['nw', 'w', 'sw', 'e', 'e']);
    assertEquals(parseLine('sesenwnenenewseeswwswswwnenewsewsw'), ['se', 'se', 'nw', 'ne', 'ne', 'ne', 'w', 'se', 'e', 'sw', 'w', 'sw', 'sw', 'w', 'ne', 'ne', 'w', 'se', 'w', 'sw']);
});
