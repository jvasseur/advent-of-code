enum Direction {
    E = 'e',
    SE = 'se',
    SW = 'sw',
    W = 'w',
    NW = 'nw',
    NE = 'ne',
}

interface Canonical {
    [Direction.E]: number,
    [Direction.NE]: number,
}

export type { Canonical };
export { Direction };
