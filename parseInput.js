import fs from 'fs';

const identity = (value) => value;

const types = {
    char: {
        regex: '\\w',
        parser: identity,
    },
    int: {
        regex: '\\d+',
        parser: (value) => parseInt(value, 10),
    },
    string: {
        regex: '\\w+',
        parser: identity,
    },
};

const parseFormat = (format) => {
    const parsers = {};
    const regexpString = format.replace(/\{(?<name>\w+):(?<type>\w+)\}/g, (match, name, type) => {
        parsers[name] = types[type].parser;

        return `(?<${name}>${types[type].regex})`;
    });

    const regexp = new RegExp(regexpString);

    return (line) => {
        const { groups } = regexp.exec(line);

        return Object.fromEntries(Object.entries(groups).map(([name, value]) => [name, parsers[name](value)]));
    };
}

const parseInput = (path, format) => {
    const parser = parseFormat(format);

    const data = fs.readFileSync(path, 'utf8');

    const lines = data.split('\n').filter((line) => line !== '');

    return lines.map(parser);
};

export default parseInput;