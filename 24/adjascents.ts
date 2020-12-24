import TileSet from './TileSet.ts';
import { Canonical } from './types.ts';

const adjascents = [
    [1, 0],
    [0, 1],
    [-1, 1],
    [-1, 0],
    [0, -1],
    [1, -1],
];

const getAdjascents = (tile: Canonical): Canonical[] => adjascents.map(([de, dne]) => ({
    e: tile.e + de,
    ne: tile.ne + dne,
}));

const getAllAdjascents = (set: TileSet): TileSet => {
    const tiles = new TileSet();

    for (const tile of set) {
        for (const adjascent of getAdjascents(tile)) {
            tiles.add(adjascent);
        }
    }

    return tiles;
};

const countAdjascents = (tile: Canonical, set: TileSet): number => {
    let count = 0;

    for (const adjascentTile of getAdjascents(tile)) {
        if (set.has(adjascentTile)) {
            count++;
        }
    }

    return count;
};

export { countAdjascents, getAdjascents, getAllAdjascents };
