import applyMask from './applyMask.ts';
import applyMaskV2 from './applyMaskV2.ts';

const input: string[] = (await Deno.readTextFile('input.txt')).split('\n').filter((line: string) => line !== '');

interface MaskInstruction {
    type: 'mask',
    value: string,
}

interface MemInstruction {
    type: 'mem',
    index: number,
    value: number,
}

type Instruction = MaskInstruction | MemInstruction;

const instructions: Instruction[] = input.map((line: string) => {
    const maskResult = /^mask = ([0-1X]+)$/.exec(line);
    if (maskResult) {
        return {
            type: 'mask',
            value: maskResult[1],
        }
    }

    const memResult = /^mem\[([0-9]+)\] = ([0-9]+)$/.exec(line);
    if (memResult) {
        return {
            type: 'mem',
            index: parseInt(memResult[1], 10),
            value: parseInt(memResult[2], 10),
        }
    }

    throw new Error(`Invalid line: "${line}"`);
});

{
    let mask: string|null = null;
    let mem: Record<number, number> = {};
    for (const instruction of instructions) {
        if (instruction.type === 'mask') {
            mask = instruction.value;
        }

        if (instruction.type === 'mem') {
            if (!mask) {
                throw new Error('Uninitialized mask');
            }

            mem[instruction.index] = applyMask(instruction.value, mask);
        }
    }

    console.log([...Object.values(mem)].reduce((a, b) => a + b, 0));
}

{
    let mask: string|null = null;
    let mem: Record<number, number> = {};
    for (const instruction of instructions) {
        if (instruction.type === 'mask') {
            mask = instruction.value;
        }

        if (instruction.type === 'mem') {
            if (!mask) {
                throw new Error('Uninitialized mask');
            }

            for (const index of applyMaskV2(instruction.index, mask)) {
                mem[index] = instruction.value;
            }
        }
    }

    console.log([...Object.values(mem)].reduce((a, b) => a + b, 0));
}
