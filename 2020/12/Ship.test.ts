import { assertEquals } from 'https://deno.land/std@0.80.0/testing/asserts.ts';
import Ship from './Ship.ts';

Deno.test('example', () => {
    const ship = new Ship();

    ship.moveForward(10);
    ship.moveNorth(3);
    ship.moveForward(7);
    ship.turnRight(90);
    ship.moveForward(11);

    assertEquals(ship.east, 17);
    assertEquals(ship.north, -8);
});
