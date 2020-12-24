import { assertEquals } from 'https://deno.land/std@0.80.0/testing/asserts.ts';
import TileSet from './TileSet.ts';

Deno.test('TileSet.size', () => {
    const set = new TileSet();

    set.add({ e: 0, ne: 0 });
    set.add({ e: 1, ne: 0 });

    assertEquals(set.size, 2);
});

Deno.test('TileSet.add duplicate', () => {
    const set = new TileSet();

    set.add({ e: 0, ne: 0 });
    set.add({ e: 0, ne: 0 });

    assertEquals(set.size, 1);
});

Deno.test('TileSet.delete', () => {
    const set = new TileSet();

    set.add({ e: 0, ne: 0 });
    set.delete({ e: 0, ne: 0 });

    assertEquals(set.size, 0);
});

Deno.test('TileSet.has', () => {
    const set = new TileSet();

    set.add({ e: 0, ne: 0 });

    assertEquals(set.has({ e: 0, ne: 0 }), true);
    assertEquals(set.has({ e: 0, ne: 1 }), false);
});
