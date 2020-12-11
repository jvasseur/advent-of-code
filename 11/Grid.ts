const directions = [
    [-1, -1],
    [1, 1],
    [1, -1],
    [-1, 1],
    [0, -1],
    [-1, 0],
    [0, 1],
    [1, 0],
];

class Grid {
    #data: string[][];

    constructor(data: string[][]) {
        // Clone input data for imutability
        this.#data = data.map((line) => [...line]);
    }

    /**
     * Returns true if a position is valid.
     */
    isValid(i: number, j: number): boolean {
        return i >= 0 && i < this.#data.length && j >= 0 && j < this.#data[i].length;
    }

    isSeat(i: number, j: number): boolean {
        return this.#data[i][j] !== '.';
    }

    isEmptySeat(i: number, j: number): boolean {
        return this.#data[i][j] === 'L';
    }

    isOccupiedSeat(i: number, j: number): boolean {
        return this.#data[i][j] === '#';
    }

    setOccupied(i: number, j: number): void {
        if (!this.isSeat(i, j)) {
            throw new Error('Invalid seat position');
        }

        this.#data[i][j] = '#';
    }

    setEmpty(i: number, j: number): void {
        if (!this.isSeat(i, j)) {
            throw new Error('Invalid seat position');
        }

        this.#data[i][j] = 'L';
    }

    countOccupied(): number {
        let occupied = 0;

        this.forEach((i, j) => {
            if (this.isOccupiedSeat(i, j)) {
                occupied++;
            }
        });

        return occupied;
    }

    countAdjacentOccupied(i: number, j: number): number {
        let occupied = 0;

        for (let i2 = i - 1; i2 <= i + 1; i2++) {
            for (let j2 = j - 1; j2 <= j + 1; j2++) {
                if ((i === i2 && j === j2) || !this.isValid(i2, j2)) {
                    continue;
                }

                if (this.isOccupiedSeat(i2, j2)) {
                    occupied++;
                }
            }
        }

        return occupied;
    }

    countVisibleOccupied(i: number, j: number): number {
        let occupied = 0;

        for (const [di, dj] of directions) {
            let i2 = i;
            let j2 = j;

            while (true) {
                i2 = i2 + di;
                j2 = j2 + dj;

                if (!this.isValid(i2, j2)) {
                    break;
                }

                if (this.isSeat(i2, j2)) {
                    if (this.isOccupiedSeat(i2, j2)) {
                        occupied++;
                    }

                    break;
                }
            }
        }

        return occupied;
    }

    isEquals(grid: Grid): boolean {
        for (let i = 0; i < this.#data.length; i++) {
            for (let j = 0; j < this.#data[i].length; j++) {
                if (this.#data[i][j] !== grid.#data[i][j]) {
                    return false;
                }
            }
        }

        return true;
    }

    forEach(callback: (i: number, j: number) => void): void {
        for (let i = 0; i < this.#data.length; i++) {
            for (let j = 0; j < this.#data[i].length; j++) {
                callback(i, j);
            }
        }
    }

    toString(): string {
        return this.#data.map((line) => line.join('')).join('\n');
    }

    static fromGrid(grid: Grid): Grid {
        return new Grid(grid.#data);
    }

    static fromString(input: string): Grid {
        return new Grid(input.split('\n').filter((line) => line !== '').map((line) => line.split('')));
    }
}

export default Grid;
