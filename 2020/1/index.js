const input = (await Deno.readTextFile(new URL('input.txt', import.meta.url))).split('\n').map((value) => parseInt(value, 10));

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
