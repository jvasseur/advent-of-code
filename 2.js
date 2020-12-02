import fs from 'fs';

const input = fs.readFileSync('2.txt', 'utf8').split('\n')
    .filter((line) => line !== '')
    .map((line) => /^(?<min>\d+)-(?<max>\d+) (?<letter>\w): (?<password>\w+)$/.exec(line))
    .map((line) => line.groups)
    .map((line) => ({
        ...line,
        min: parseInt(line.min, 10),
        max: parseInt(line.max, 10)
    }));

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
    console.log({ min, max, letter, password });
    console.log(password.charAt(min - 1));
    console.log(password.charAt(max - 1));
    if ((password.charAt(min - 1) === letter) ^ (password.charAt(max - 1) === letter)) {
        valid2++;
    }
}

console.log(valid2);