import { Canonical } from './types.ts';

const stringify = (tile: Canonical): string => `${tile.e},${tile.ne}`;

class TileSet implements Iterable<Canonical> {
    #inner = new Map<string, Canonical>();

    add(value: Canonical): this {
        this.#inner.set(stringify(value), value);

        return this;
    }

    clear(): void {
        this.#inner.clear();
    }

    delete(value: Canonical): boolean {
        return this.#inner.delete(stringify(value));
    }

    forEach(callbackfn: (value: Canonical, value2: Canonical, set: TileSet) => void, thisArg?: any): void {
        this.#inner.forEach((value) => {
            callbackfn(value, value, this);
        });
    }

    has(value: Canonical): boolean {
        return this.#inner.has(stringify(value));
    }

    get size(): number {
        return this.#inner.size;
    }

    *[Symbol.iterator](): Iterator<Canonical> {
        for (const tile of this.#inner.values()) {
            yield tile;
        }
    }
}

export default TileSet;
