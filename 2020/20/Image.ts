import { Tile } from './Tile.ts';
import Picture from './Picture.ts'

const adjascents: [-1|0|1, -1|0|1][] = [
    [1, 0],
    [0, 1],
    [-1, 0],
    [0, -1],
];

const edge = {
    [-1]: 0,
    1: 9,
};

const adjascentEdge = {
    [-1]: 9,
    1: 0,
};

class Image implements Picture {
    #data: Tile[][] = [...Array(12)].map(() => [...Array(12)]);

    hasTile(x: number, y: number): boolean {
        return x >= 0 && x <= 11 && y >= 0 && y <= 11 && !!this.#data[11 - y][x];
    }

    getTile(x: number, y: number): Tile {
        return this.#data[11 - y][x];
    }

    hasAdjascent(x: number, y: number): boolean {
        return adjascents.some(([dx, dy]) => this.hasTile(x + dx, y + dy));
    }

    get(x: number, y: number): string {
        const tyleX = Math.floor(x / 8);
        const tyleY = Math.floor(y / 8);

        const tyle = this.getTile(tyleX, tyleY)

        return tyle.get(x % 8 + 1, y % 8 + 1);
    }

    fit(x: number, y: number, tile: Tile): boolean {
        return adjascents.every(([dx, dy]) => {
            if (!this.hasTile(x + dx, y + dy)) {
                return true;
            }

            const adjascentTile = this.getTile(x + dx, y + dy);

            if (dx !== 0) {
                for (let y = 0; y < 10; y++) {
                    if (tile.get(edge[dx], y) !== adjascentTile.get(adjascentEdge[dx], y)) {
                        return false;
                    }
                }

                return true;
            }

            if (dy !== 0) {
                for (let x = 0; x < 10; x++) {
                    if (tile.get(x, edge[dy]) !== adjascentTile.get(x, adjascentEdge[dy])) {
                        return false;
                    }
                }

                return true;
            }
        });
    }

    toString(): string {
        return this.#data.map((line) => line.map((tile) => tile ? tile.id : '    ').join(' ')).join('\n');
    }

    clone(): Image {
        const clone = new Image();

        for (let x = 0; x < 12; x++) {
            for (let y = 0; y < 12; y++) {
                if (this.hasTile(x, y)) {
                    clone.setTile(x, y, this.getTile(x, y));
                }
            }
        }

        return clone;
    }

    withTile(x: number, y: number, tile: Tile): Image {
        const clone = this.clone();

        clone.setTile(x, y, tile);

        return clone;
    }

    private setTile(x: number, y: number, tile: Tile): void {
        if (this.hasTile(x, y)) {
            throw new Error('Trying to fill an already filled tile');
        }

        if (!this.fit(x, y, tile)) {
            throw new Error('Tile does not fit');
        }

        this.#data[11 - y][x] = tile;
    }
}

export default Image;
