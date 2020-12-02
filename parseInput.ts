const identity = (value: string): string => value;

const types: Record<string, { regex: string, parser: (value: string) => any }> = {
    char: {
        regex: '\\w',
        parser: identity,
    },
    int: {
        regex: '\\d+',
        parser: (value :string): number => parseInt(value, 10),
    },
    string: {
        regex: '\\w+',
        parser: identity,
    },
};

const parseFormat = (format: string): (line: string) => Record<string, any> => {
    const parsers: Record<string, (value: string) => any> = {};
    const regexpString = format.replace(/\{(?<name>\w+):(?<type>\w+)\}/g, (match: string, name: string, type: string): string => {
        if (!(type in types)) {
            throw new Error('Invalid type');
        }

        parsers[name] = types[type].parser;

        return `(?<${name}>${types[type].regex})`;
    });

    const regexp = new RegExp(`^${regexpString}$`);

    return (line: string): Record<string, any> => {
        const result = regexp.exec(line);

        if (!result || !result.groups) {
            throw new Error('Invalid line');
        }

        const { groups } = result;

        return Object.fromEntries(Object.entries(groups).map(([name, value]) => [name, parsers[name](value)]));
    };
}

const parseInput = async (path: string, format: string): Promise<(Record<string, any>)[]> => {
    const parser = parseFormat(format);

    const data = await Deno.readTextFile(path);

    const lines = data.split('\n').filter((line) => line !== '');

    return lines.map(parser);
};

export default parseInput;
