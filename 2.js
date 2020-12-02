import parseInput from './parseInput.js';

const input = parseInput('2.txt', '{min:int}-{max:int} {letter:char}: {password:string}');

let valid = 0;

for (const { min, max, letter, password } of input) {
    let count = 0;

    for (const passwordLetter of password) {
        if (passwordLetter === letter) {
            count++;
        }
    }

    if (count >= min && count <= max) {
        valid++;
    }
}

console.log(valid);

let valid2 = 0;

for (const { min, max, letter, password } of input) {
    if ((password.charAt(min - 1) === letter) ^ (password.charAt(max - 1) === letter)) {
        valid2++;
    }
}

console.log(valid2);