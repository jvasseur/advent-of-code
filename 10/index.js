const input = (await Deno.readTextFile(new URL('input.txt', import.meta.url))).split('\n').filter((line) => line !== '');

const adapters = input.map((adapter) => parseInt(adapter, 10));

adapters.sort((a, b) => a - b);

let jolt = 0;

const diff = {
    1: 0,
    2: 0,
    3: 0,
};

for (const adapter of adapters) {
    diff[adapter - jolt]++;

    jolt = adapter;
}

console.log(diff[1] * (diff[3] + 1));

const paths = {};

const [first, ...rest] = adapters.reverse();

paths[first] = 1;

for (const adapter of [...rest, 0]) {
    let count = 0;

    for (let i = adapter; i <= adapter + 3; i++) {
        if (i in paths) {
            count += paths[i];
        }
    }

    paths[adapter] = count;
}

console.log(paths[0]);
