import turn from './turn.ts';

const transform = (subject: number, loopSize: number): number => {
    let value = 1;
    for (let i = 0; i < loopSize; i++) {
        value = turn(value, subject);
    }

    return value;
};

export default transform;
