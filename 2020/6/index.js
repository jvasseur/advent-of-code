import reduceAll from '../utils/reduceAll.ts'

const intersect = (a, b) => a.filter((value) => b.includes(value));

const input = await Deno.readTextFile(new URL('input.txt', import.meta.url));

const groups = input.split('\n\n').map((group) => group.split('\n').filter((person) => person !== '').map((person) => person.split('')));

const groupOneAnswers = groups.map((group) => [...group.reduce((accumulator, current) => new Set([...accumulator, ...current]), new Set())]);
const sumOne = groupOneAnswers.flat().length;

console.log(sumOne);

const groupAllAnswers = groups.map((group) => reduceAll((accumulator, current) => intersect(accumulator, current), group));
const sumAll = groupAllAnswers.flat().length;

console.log(sumAll);
