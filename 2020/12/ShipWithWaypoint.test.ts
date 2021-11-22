import { assertEquals } from 'https://deno.land/std@0.80.0/testing/asserts.ts';
import ShipWithWaypoint from './ShipWithWaypoint.ts';

Deno.test('example', () => {
    const ship = new ShipWithWaypoint();

    ship.moveForward(10);
    ship.moveWaypointNorth(3);
    ship.moveForward(7);
    ship.turnWaypointRight(90);
    ship.moveForward(11);

    assertEquals(ship.east, 214);
    assertEquals(ship.north, -72);
});
