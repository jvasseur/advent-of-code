import fs from 'fs';

const input = fs.readFileSync('1.txt', 'utf8').split('\n').map((value) => parseInt(value, 10));

for (const a of input) {
    for (const b of input) {
        if (2020 === a + b) {
            console.log(a, b);
            console.log(a * b);
        }
    }
}

for (const a of input) {
    for (const b of input) {
        for (const c of input) {
            if (2020 === a + b + c) {
                console.log(a, b, c);
                console.log(a * b * c);
            }
        }
    }
}