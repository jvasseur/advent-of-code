import { countAdjascents, getAllAdjascents } from './adjascents.ts';
import canonicalize from './canonicalize.ts';
import parseLine from './parseLine.ts';
import TileSet from './TileSet.ts';

const input = await Deno.readTextFile(new URL('input.txt', import.meta.url));
const lines = input.replace(/\n$/, '').split('\n');

const tiles = lines.map((line) => canonicalize(parseLine(line)));

const set = new TileSet();

for (const tile of tiles) {
    if (set.has(tile)) {
        set.delete(tile);
    } else {
        set.add(tile);
    }
}

console.log(set.size);

let current = set;

for (let i = 0; i < 100; i++) {
    const next = new TileSet();

    for (const tile of getAllAdjascents(current)) {
        const adjascentsCount = countAdjascents(tile, current);

        if (current.has(tile)) {
            if (adjascentsCount === 0 || adjascentsCount > 2) {
                // Flip to white
            } else {
                next.add(tile);
            }
        } else {
            if (adjascentsCount === 2) {
                next.add(tile);
            }
        }
    }

    current = next;
}

console.log(current.size);
