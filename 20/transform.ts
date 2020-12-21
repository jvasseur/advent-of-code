
import { Tile, Rotation, Flip, RotatedTile, FlippedTile } from './Tile.ts';

const rotations = [null, Rotation.Rotate90, Rotation.Rotate180, Rotation.Rotate270];
const flips = [null, Flip.VerticalFlip, Flip.HorizontalFlip];

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
