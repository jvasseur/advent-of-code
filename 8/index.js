import parseInput from '../utils/parseInput.ts';

const input = await parseInput(new URL('input.txt', import.meta.url), '{operation:string} {argument:int}');

const run = (instructions = input) => {
    let accumulator = 0;
    let instruction = 0;

    const visited = new Set();

    while (!visited.has(instruction) && instruction < instructions.length) {
        visited.add(instruction);

        const { operation, argument } = instructions[instruction];

        switch (operation) {
            case 'acc':
                accumulator+=argument;
                instruction++;
                break;
            case 'jmp':
                instruction+=argument;
                break;
            case 'nop':
                instruction++;
                break;
        }
    }

    return {
        accumulator,
        instruction,
        visited,
    }
}

console.log(run().accumulator);

const reverser = {
    jmp: 'nop',
    nop: 'jmp',
};

for (const [index, { operation, argument }] of input.entries()) {
    if (operation === 'acc') {
        continue;
    }

    const replaced = [...input];

    replaced[index] = {
        operation: reverser[operation],
        argument,
    };

    const result = run(replaced);

    if (result.instruction === input.length) {
        console.log(result.accumulator);

        break;
    }
}
