import Grid from './11/Grid.ts';

const input = await Deno.readTextFile('11.txt');

const inputGrid = Grid.fromString(input);

const round1 = (inGrid) => {
    const outGrid = Grid.fromGrid(inGrid);

    inGrid.forEach((i, j) => {
        if (inGrid.isEmptySeat(i, j) && inGrid.countAdjacentOccupied(i, j) === 0) {
            outGrid.setOccupied(i, j);
        }

        if (inGrid.isOccupiedSeat(i, j) && inGrid.countAdjacentOccupied(i, j) >= 4) {
            outGrid.setEmpty(i, j);
        }
    });

    return outGrid;
}

let current1 = inputGrid;
let previous1;

do {
    previous1 = current1;
    current1 = round1(current1);
} while (!current1.isEquals(previous1));

console.log(current1.countOccupied());

const round2 = (inGrid) => {
    const outGrid = Grid.fromGrid(inGrid);

    inGrid.forEach((i, j) => {
        if (inGrid.isEmptySeat(i, j) && inGrid.countVisibleOccupied(i, j) === 0) {
            outGrid.setOccupied(i, j);
        }

        if (inGrid.isOccupiedSeat(i, j) && inGrid.countVisibleOccupied(i, j) >= 5) {
            outGrid.setEmpty(i, j);
        }
    });

    return outGrid;
}

let current2 = inputGrid;
let previous2;

do {
    previous2 = current2;
    current2 = round2(current2);
} while (!current2.isEquals(previous2));

console.log(current2.countOccupied());
