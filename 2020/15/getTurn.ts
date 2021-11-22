const getTurn = (initialTurns: number[], turn: number): number => {
    const seen = new Uint32Array(turn);

    const init = [...initialTurns];

    let previous = init.pop() as number;

    for (const [index, number] of init.entries()) {
        seen[number] = index + 1;
    }

    for (let i = initialTurns.length; i < turn; i++) {
        const next = seen[previous] ? i - seen[previous] : 0;

        seen[previous] = i;

        previous = next;
    }

    return previous;
}

export default getTurn;
