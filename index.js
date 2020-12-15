import { exists } from 'https://deno.land/std@0.80.0/fs/exists.ts';

const range = (start, end) => [...Array(end - start + 1)].map((el, ind) => ind + start);

await Promise.all(range(1, 15).map(async (i) => {
    if (await exists(`./${i}/index.ts`)) {
        await import(`./${i}/index.ts`);
    } else {
        await import(`./${i}/index.js`);
    }
}));
