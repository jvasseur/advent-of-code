import { Rule } from './types.ts';

const subValidation = (rules: Rule[], id: number, message: string, start: number): false|number => {
    const rule = rules[id];

    if (typeof rule === 'string') {
        if (message[start] === rule) {
            return start + 1;
        } else {
            return false;
        }
    }

    for (const option of rule) {
        let index = start;
        let valid = true;

        for (const part of option) {
            const result = subValidation(rules, part, message, index);

            if (result === false) {
                valid = false;

                break;
            }

            index = result;
        }

        if (valid) {
            return index;
        }
    }

    return false;
};

const isValid = (rules: Rule[], id: number, message: string): boolean => subValidation(rules, id, message, 0) === message.length;

export default isValid;
