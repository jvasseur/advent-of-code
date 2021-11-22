import HyperSpace from './HyperSpace.ts';
import Space from './Space.ts';

const input = await Deno.readTextFile(new URL('input.txt', import.meta.url));

const lines = input.split('\n').filter((line) => line !== '');

const space = new Space();
const hyperSpace = new HyperSpace();

for (const [x, line] of lines.entries()) {
    for (const [y, char] of line.split('').entries()) {
        if (char === '#') {
            space.set(x, y, 0, true);
            hyperSpace.set(x, y, 0, 0, true);
        }
    }
}

const bootedSpace = space.nextSpaceForCycles(6);

console.log(bootedSpace.countActive());

const bootedHyperSpace = hyperSpace.nextSpaceForCycles(6);

console.log(bootedHyperSpace.countActive());
