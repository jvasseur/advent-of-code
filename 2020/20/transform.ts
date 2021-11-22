
import { Tile, RotatedTile, FlippedTile } from './Tile.ts';
import { rotations, flips } from './transformations.ts';

const transform = (tile: Tile): Tile[] => {
    let transformations: Tile[] = [];

    for (const rotation of rotations) {
        const rotatedTile = rotation !== null ? new RotatedTile(tile, rotation) : tile;

        for (const flip of flips) {
            const flippedTile = flip !== null ? new FlippedTile(rotatedTile, flip) : rotatedTile;

            transformations = [...transformations, flippedTile];
        }
    }

    return transformations;
}

export default transform;
