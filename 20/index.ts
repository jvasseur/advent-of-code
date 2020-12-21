import parseInt10 from '../utils/parseInt10.ts'
import { Tile, BaseTile } from './Tile.ts';
import Image from './Image.ts';
import transform from './transform.ts';

const size = 12;
const input = (await Deno.readTextFile(new URL('input.txt', import.meta.url))).replace(/\n+$/, '');

const tiles: Tile[] = input.split('\n\n').map((tileInput) => {
    const [header, data] = tileInput.split(':\n');

    const [, id] = header.split(' ');

    return new BaseTile(parseInt10(id), data.split('\n').map((line) => line.split('')));
});

const fitTile = (tile: Tile, image: Image, x: number, y: number): Tile[] => transform(tile)
    .filter((transformation) => image.fit(x, y, transformation));

class InvalidError extends Error {}

const fitTiles = (initial: Image, tiles: Tile[]): [Image, Tile[]] => {
    let image = initial;
    let remaining = tiles;

    while (remaining.length > 0) {
        const count = remaining.length;

        const fits: Tile[][][] = [...Array(size)].map(() => [...Array(size)].map(() => []));

        for (let x = 0; x < size; x++) {
            for (let y = 0; y < size; y++) {
                if (image.hasTile(x, y) || !image.hasAdjascent(x, y)) {
                    continue;
                }

                let fit: Tile[] = [];

                for (const tile of remaining) {
                    fit = [...fit, ...fitTile(tile, image, x, y)];
                }

                if (fit.length === 0) {
                    throw new InvalidError();
                }

                if (fit.length === 1) {
                    image = image.withTile(x, y, fit[0]);
                    remaining = remaining.filter((i) => i.id !== fit[0].id);
                }

                fits[x][y] = fit;
            }
        }

        if (remaining.length === count) {
            let min = 1000;
            let minX = 0;
            let minY = 0;

            for (let x = 0; x < size; x++) {
                for (let y = 0; y < size; y++) {
                    const length = fits[x][y].length;

                    if (length !== 0 && length < min) {
                        min = length;
                        minX = x;
                        minY = y;
                    }
                }
            }

            if (min === 1000) {
                throw new Error();
            }

            for (const tile of fits[minX][minY]) {
                const newImage = image.withTile(minX, minY, tile);
                const newRemaining = remaining.filter((i) => i.id !== tile.id);

                console.log('=======');
                console.log(newImage.toString());

                try {
                    [image, remaining] = fitTiles(newImage, newRemaining);
                } catch (error) {
                    if (error instanceof InvalidError) {
                        continue;
                    }

                    throw error;
                }

                if (remaining.length === 0) {
                    return [image, remaining];
                }
            }

            throw new InvalidError();
        }
    }

    return [image, remaining];
}


const resolve = (): Image => {
    let image = new Image();

    for (const tile of tiles) {
        const remaining = tiles.filter((i) => i.id !== tile.id);

        for (const transformation of transform(tile)) {
            try {
                [image] = fitTiles(image.withTile(8, 8, transformation), remaining);

                return image;
            } catch (error) {
                if (error instanceof InvalidError) {
                    continue;
                }

                throw error;
            }
        }
    }

    throw new Error('Not found');
}

const result = resolve();

console.log('=======');
console.log(result.toString());
console.log(
    result.getTile(0, 0).id *
    result.getTile(0, size - 1).id *
    result.getTile(size - 1, 0).id *
    result.getTile(size - 1, size - 1).id
);
