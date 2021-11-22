interface Range {
    min: number,
    max: number,
}

interface Field {
    name: string,
    ranges: Range[],
}

export type { Range, Field };
