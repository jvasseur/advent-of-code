const range = (start, end) => [...Array(end-start+1)].map((el, ind) => ind + start);

const exists = async (filePath) => {
    try {
        await Deno.lstat(filePath);
        return true;
    } catch (err) {
        if (err instanceof Deno.errors.NotFound) {
            return false;
        }

        throw err;
    }
}

await Promise.all(range(1, 9).map(async (i) => {
    if (await exists(`./${i}.ts`)) {
        await import(`./${i}.ts`);
    } else {
        await import(`./${i}.js`);
    }
}));
