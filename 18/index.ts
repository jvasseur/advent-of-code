import compute1 from './part1/compute.ts';
import compute2 from './part2/compute.ts';

const input = await Deno.readTextFile(new URL('input.txt', import.meta.url));

const lines = input.split('\n').filter((line) => line !== '');

const values1 = lines.map(compute1);

console.log(values1.reduce((a, b) => a + b, 0));

const values2 = lines.map(compute2);

console.log(values2.reduce((a, b) => a + b, 0));
