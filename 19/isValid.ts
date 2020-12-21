import { Rule } from './types.ts';

const subValidation = (rules: Rule[], id: number, message: string, start: number[]): number[] => {
    const rule = rules[id];

    if (typeof rule === 'string') {
        const indexes = start
            .filter((index) => message[index] === rule)
            .map((index) => index + 1);

        return indexes;
    }

    let indexes: number[] = [];

    for (const option of rule) {
        let index = start;
        let valid = true;

        for (const part of option) {
            const result = subValidation(rules, part, message, index);

            if (result.length === 0) {
                valid = false;

                break;
            }

            index = result;
        }

        if (valid) {
            indexes = [
                ...indexes,
                ...index,
            ];
        }
    }

    return indexes;
};

const isValid = (rules: Rule[], id: number, message: string): boolean => subValidation(rules, id, message, [0]).some((index) => index === message.length);

export default isValid;
