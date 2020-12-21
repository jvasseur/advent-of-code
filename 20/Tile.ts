interface Tile {
    id: number;
    get(x: number, y: number): string;
    toString(): string;
}

const equals = (a: Tile, b: Tile): boolean => {
    for (let x = 0; x < 10; x++) {
        for (let y = 0; y < 10; y++) {
            if (a.get(x, y) !== b.get(x, y)) {
                return false;
            }
        }
    }

    return true;
}

class BaseTile implements Tile {
    #id: number;
    #data: string[][];

    constructor(id: number, data: string[][]) {
        if (data.length !== 10 || data.some((line) => line.length !== 10)) {
            throw new Error('Invalid data');
        }

        this.#id = id;
        this.#data = data;
    }

    get id() {
        return this.#id;
    }

    get(x: number, y: number): string {
        return this.#data[9 - y][x];
    }
}

enum Rotation {
    Rotate90,
    Rotate180,
    Rotate270,
}

class RotatedTile implements Tile {
    #parent: Tile;
    #rotation: Rotation;

    constructor(parent: Tile, roation: Rotation) {
        this.#parent = parent;
        this.#rotation = roation;
    }

    get id() {
        return this.#parent.id;
    }

    get(x: number, y: number): string {
        switch (this.#rotation) {
            case Rotation.Rotate90:
                return this.#parent.get(y, 9 - x);
            case Rotation.Rotate180:
                return this.#parent.get(9 - x, 9 - y);
            case Rotation.Rotate270:
                return this.#parent.get(9 - y, x);
        }
    }
}

enum Flip {
    VerticalFlip,
    HorizontalFlip,
}

class FlippedTile implements Tile {
    #parent: Tile;
    #flip: Flip;

    constructor(parent: Tile, flip: Flip) {
        this.#parent = parent;
        this.#flip = flip;
    }

    get id() {
        return this.#parent.id;
    }

    get(x: number, y: number): string {
        switch (this.#flip) {
            case Flip.VerticalFlip:
                return this.#parent.get(x, 9 - y);
            case Flip.HorizontalFlip:
                return this.#parent.get(9 - x, y);
        }
    }
}

export type { Tile };
export { BaseTile, equals, RotatedTile, Rotation, Flip, FlippedTile };
