import { Canonical, Direction } from './types.ts';

const canonicalize = (directions: Direction[]): Canonical => {
    const counts = Object.fromEntries((Object.values(Direction)).map((direction) => [direction, 0])) as Record<Direction, number>;

    for (const direction of directions) {
        counts[direction]++;
    }

    const reduced = {
        [Direction.E]: counts[Direction.E] - counts[Direction.W],
        [Direction.NE]: counts[Direction.NE] - counts[Direction.SW],
        [Direction.NW]: counts[Direction.NW] - counts[Direction.SE],
    };

    // NW = W + NE
    const canonical = {
        [Direction.E]: reduced[Direction.E] - reduced[Direction.NW],
        [Direction.NE]: reduced[Direction.NE] + reduced[Direction.NW],
    };

    return canonical;
};

export default canonicalize;
