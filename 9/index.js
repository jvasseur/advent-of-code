const input = (await Deno.readTextFile(new URL('input.txt', import.meta.url))).split('\n').filter((line) => line !== '');

const numbers = input.map((number) => parseInt(number, 10));

const isValid = (number, queue) => {
    for (const a of queue) {
        for (const b of queue) {
            if (number === a + b) {
                return true;
            }
        }
    }

    return false;
};

const queue = [];
let invalid;

for (const number of numbers) {
    if (queue.length < 25) {
        queue.push(number);

        continue;
    }

    if (!isValid(number, queue)) {
        invalid = number;

        break;
    }

    queue.shift(number);
    queue.push(number);
}

console.log(invalid);

let vulnerability;

for (const [key, number] of numbers.entries()) {
    let sum = 0;
    const suite = [];

    for (let index = key; index < numbers.length && sum < invalid; index++) {
        suite.push(numbers[index]);

        sum += numbers[index];
    }

    if (sum === invalid) {
        vulnerability = suite;

        break;
    }
}

console.log(Math.min(...vulnerability) + Math.max(...vulnerability));
