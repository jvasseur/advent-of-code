import { assertEquals } from 'https://deno.land/std@0.80.0/testing/asserts.ts';
import HyperSpace from './HyperSpace.ts';

Deno.test('example', () => {
    const space = new HyperSpace();

    // .#.
    space.set(-1, -1, 0, 0, false);
    space.set(-1, 0, 0, 0, true);
    space.set(-1, 1, 0, 0, false);
    // ..#
    space.set(0, -1, 0, 0, false);
    space.set(0, 0, 0, 0, false);
    space.set(0, 1, 0, 0, true);
    // ###
    space.set(1, -1, 0, 0, true);
    space.set(1, 0, 0, 0, true);
    space.set(1, 1, 0, 0, true);

    assertEquals(space.countActive(), 5);

    const bootedSpace = space.nextSpaceForCycles(6);

    assertEquals(bootedSpace.countActive(), 848);
});
