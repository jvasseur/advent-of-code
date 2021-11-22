class Space {
    #data: Map<number, Map<number, Map<number, boolean>>> = new Map();

    #minX: number = 0;
    #maxX: number = 0;
    #minY: number = 0;
    #maxY: number = 0;
    #minZ: number = 0;
    #maxZ: number = 0;

    constructor() {
    }

    nextSpace(): Space {
        const space = new Space();

        for (let x = this.#minX - 1; x <= this.#maxX + 1; x++) {
            for (let y = this.#minY - 1; y <= this.#maxY + 1; y++) {
                for (let z = this.#minZ - 1; z <= this.#maxZ + 1; z++) {
                    const activeNeighbors = this.countActiveNeighbors(x, y, z);

                    if (this.get(x, y, z)) {
                        if (activeNeighbors === 2 || activeNeighbors === 3) {
                            space.set(x, y, z, true);
                        }
                    } else {
                        if (activeNeighbors === 3) {
                            space.set(x, y, z, true);
                        }
                    }
                }
            }
        }

        return space;
    }

    nextSpaceForCycles(cycle: number): Space {
        let space: Space = this;

        for (let i = 0; i < cycle; i++) {
            space = space.nextSpace();
        }

        return space;
    }

    countActive(): number {
        let count = 0;

        for (const yMap of this.#data.values()) {
            for (const zMap of yMap.values()) {
                for (const value of zMap.values()) {
                    if (value) {
                        count++;
                    }
                }
            }
        }

        return count;
    }

    countActiveNeighbors(x: number, y: number, z :number): number {
        let count = 0;

        for (let dx = -1; dx <= 1; dx++) {
            for (let dy = -1; dy <= 1; dy++) {
                for (let dz = -1; dz <= 1; dz++) {
                    if (dx === 0 && dy === 0 && dz === 0) {
                        continue;
                    }

                    if (this.get(x + dx, y + dy, z + dz)) {
                        count++;
                    }
                }
            }
        }

        return count;
    }

    get(x: number, y: number, z :number): boolean {
        const yMap = this.#data.get(x);

        if (yMap === undefined) {
            return false;
        }

        const zMap = yMap.get(y);

        if (zMap === undefined) {
            return false;
        }

        const value = zMap.get(z);

        if (value === undefined) {
            return false;
        }

        return value;
    }

    set(x: number, y: number, z :number, value: boolean): void {
        let yMap = this.#data.get(x);

        if (yMap === undefined) {
            yMap = new Map();
            this.#data.set(x, yMap);
        }

        let zMap = yMap.get(y);

        if (zMap === undefined) {
            zMap = new Map();
            yMap.set(y, zMap);
        }

        zMap.set(z, value);

        this.#minX = Math.min(this.#minX, x);
        this.#maxX = Math.max(this.#maxX, x);
        this.#minY = Math.min(this.#minY, y);
        this.#maxY = Math.max(this.#maxY, y);
        this.#minZ = Math.min(this.#minZ, z);
        this.#maxZ = Math.max(this.#maxZ, z);
    }
}

export default Space;
