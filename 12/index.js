import parseInput from '../parseInput.ts';
import Ship from './Ship.ts';
import ShipWithWaypoint from './ShipWithWaypoint.ts';

const actionMap = {
    N: 'moveNorth',
    S: 'moveSouth',
    E: 'moveEast',
    W: 'moveWest',
    L: 'turnLeft',
    R: 'turnRight',
    F: 'moveForward',
};

const actionMapWithWaypoint = {
    N: 'moveWaypointNorth',
    S: 'moveWaypointSouth',
    E: 'moveWaypointEast',
    W: 'moveWaypointWest',
    L: 'turnWaypointLeft',
    R: 'turnWaypointRight',
    F: 'moveForward',
};

const input = await parseInput('input.txt', '{action:char}{value:int}');


const ship = new Ship();
const shipWithWaypoint = new ShipWithWaypoint();

for (const { action, value } of input) {
    ship[actionMap[action]](value);
    shipWithWaypoint[actionMapWithWaypoint[action]](value);
}

console.log(Math.abs(ship.east) + Math.abs(ship.north));
console.log(Math.abs(shipWithWaypoint.east) + Math.abs(shipWithWaypoint.north));
