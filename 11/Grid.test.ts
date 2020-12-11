import { assertEquals } from 'https://deno.land/std@0.80.0/testing/asserts.ts';
import Grid from './Grid.ts';

Deno.test('countOccupied', () => {
    const grid = new Grid([
        ['#', '.', '#'],
        ['#', 'L', '#'],
        ['#', '.', '#'],
        ['#', '#', '#'],
    ]);

    assertEquals(grid.countOccupied(), 9);
});

Deno.test('countAdjacentOccupied', () => {
    const grid = new Grid([
        ['#', '#', '#'],
        ['#', 'L', '#'],
        ['#', '#', '#'],
    ]);

    assertEquals(grid.countAdjacentOccupied(1, 1), 8);
});

Deno.test('countVisibleOccupied', () => {
    const grid = new Grid([
        ['#', '#', '#'],
        ['#', 'L', '#'],
        ['#', '#', '#'],
    ]);

    assertEquals(grid.countVisibleOccupied(1, 1), 8);
});

Deno.test('countVisibleOccupied', () => {
    const grid = new Grid([
        ['#', '.', '#'],
        ['#', 'L', '#'],
        ['#', '.', '#'],
        ['#', '#', '#'],
    ]);

    assertEquals(grid.countVisibleOccupied(1, 1), 7);
});
