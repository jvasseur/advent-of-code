const intersect = (a, b) => a.filter((value) => b.includes(value));

const input = await Deno.readTextFile('6.txt');

const groups = input.split('\n\n').map((group) => group.split('\n').filter((person) => person !== '').map((person) => person.split('')));

const groupOneAnswers = groups.map((group) => [...group.reduce((accumulator, current) => new Set([...accumulator, ...current]), new Set())]);
const sumOne = groupOneAnswers.flat().length;

console.log(sumOne);

const groupAllAnswers = groups.map((group) => group.reduce((accumulator, current) => accumulator ? intersect(accumulator, current) : current));
const sumAll = groupAllAnswers.flat().length;

console.log(sumAll);