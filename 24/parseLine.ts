import { Direction } from './types.ts';

const parseLine = (line: string): Direction[] => {
    const matches = line.matchAll(/e|se|sw|w|nw|ne/g);

    const directions: Direction[] = [];

    for (const match of matches) {
        directions.push(match[0] as Direction);
    }

    return directions;
};

export default parseLine;
