import parseInt10 from '../utils/parseInt10.ts';
import reduceAll from '../utils/reduceAll.ts';
import reverse from '../utils/reverse.ts';
import turn from './turn.ts';
import game from './part2/game.ts';

const getScore = (winner: number[]): number => reduceAll((a, b) => a + b, [...reverse(winner).entries()].map(([key, value]) => (key + 1) * value))

const input = await Deno.readTextFile(new URL('input.txt', import.meta.url));

const players = input.split('\n\n').map((playerInput) => {
    const [, ...cards] = playerInput.replace(/\n$/, '').split('\n');

    return cards.map(parseInt10);
}) as [number[], number[]];

{
    let [player1, player2] = players;

    while (player1.length > 0 && player2.length > 0) {
        [player1, player2] = turn(player1, player2);
    }

    const winner = player1.length > 0 ? player1 : player2;

    console.log(getScore(winner));
}

{
    const [winner, ...recursiveDecks] = game(...players);

    console.log(getScore(recursiveDecks[winner]));
}
