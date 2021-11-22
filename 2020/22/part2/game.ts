const stringify = (a: number[], b: number[]): string => [a.join(','), b.join(',')].join(':');

const memoize = <T>(callback: (a: number[], b: number[]) => T): (a: number[], b: number[]) => T => {
    const cache: Record<string, T> = {};

    const memoized = (a: number[], b: number[]): T => {
        const key = stringify(a, b);

        if (key in cache) {
            return cache[key];
        }

        const result = callback(a, b);

        cache[key] = result;

        return result;
    };

    return memoized;
};

const round = memoize((player1: number[], player2: number[]): [number[], number[]] => {
    const [played1, ...rest1] = player1;
    const [played2, ...rest2] = player2;

    let winner: 0|1;
    if (rest1.length >= played1 && rest2.length >= played2) {
        [winner] = game(rest1.slice(0, played1), rest2.slice(0, played2));
    } else {
        winner = played1 > played2 ? 0 : 1;
    }

    if (winner === 0) {
        player1 = [...rest1, played1, played2];
        player2 = rest2;
    } else {
        player1 = rest1;
        player2 = [...rest2, played2, played1];
    }

    return [player1, player2];
});

const game = memoize((player1: number[], player2: number[]): [0|1 , number[], number[]] => {
    let rounds: Record<string, true> = {};

    while (player1.length > 0 && player2.length > 0) {
        if (stringify(player1, player2) in rounds) {
            return [0, player1, player2];
        }

        rounds[stringify(player1, player2)] = true;

        [player1, player2] = round(player1, player2);
    }

    return [player1.length > 0 ? 0 : 1, player1, player2];
});

export default game;
