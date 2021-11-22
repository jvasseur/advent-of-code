import parseInt10 from '../utils/parseInt10.ts'
import isValid from './isValid.ts';
import { Rule } from './types.ts';

const input = await Deno.readTextFile(new URL('input.txt', import.meta.url));

const [rulesInput, messagesInput] = input.split('\n\n');

const rulesLines = rulesInput.split('\n').filter((line) => line !== '');
const messages = messagesInput.split('\n').filter((line) => line !== '');

const rules: Rule[] = [];

for (const rulesLine of rulesLines) {
    const [idString, spec] = rulesLine.split(':');

    const id = parseInt10(idString);

    if (spec[1] === '"') {
        rules[id] = spec[2];
    } else {
        const parts = spec.split('|');

        rules[id] = parts.map((part) => part.trim().split(' ').map(parseInt10));
    }
}

console.log(messages.filter((message) => isValid(rules, 0, message)).length);

const fixedRules = [
    ...rules,
];

fixedRules[8] = [[42], [42, 8]];
fixedRules[11] = [[42, 31], [42, 11, 31]];

console.log(messages.filter((message) => isValid(fixedRules, 0, message)).length);
