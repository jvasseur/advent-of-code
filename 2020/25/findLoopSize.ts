import turn from './turn.ts';

const findLoopSize = (key: number): number => {
    let loopSize = 0;

    let value = 1;
    while (value !== key) {
        loopSize++;

        value = turn(value, 7);
    }

    return loopSize
};

export default findLoopSize;
