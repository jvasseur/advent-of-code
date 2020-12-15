const getTurn = (initialTurns: number[], turn: number): number => {
    const seen: Record<number, number> = {};

    const init = [...initialTurns];

    let previous = init.pop() as number;

    for (const [index, number] of init.entries()) {
        seen[number] = index + 1;
    }

    for (let i = initialTurns.length + 1; i <= turn; i++) {
        const next = previous in seen ? i - 1 - seen[previous] : 0;

        seen[previous] = i - 1;

        previous = next;
    }

    return previous;
}

export default getTurn;
