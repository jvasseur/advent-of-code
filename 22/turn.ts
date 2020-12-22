const turn = (player1: number[], player2: number[]): [number[], number[]] => {
    const [played1, ...rest1] = player1;
    const [played2, ...rest2] = player2;

    if (played1 > played2) {
        return [[...rest1, played1, played2], rest2];
    } else {
        return [rest1, [...rest2, played2, played1]];
    }
};

export default turn;
