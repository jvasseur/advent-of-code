import parseInt10 from '../utils/parseInt10.ts';
import range from '../utils/range.ts';
import play from './play.ts';

const input = '459672813';

const cups = input.split('').map(parseInt10);

{
    const end = play(cups, 100);

    const index = end.indexOf(1);

    console.log([...end.slice(index + 1), ...end.slice(0, index)].join(''));
}

{
    const end = play([...cups, ...range(10, 1_000_000)], 10_000_000);

    const index = end.indexOf(1);

    console.log(end[index + 1] * end[index + 2]);
}
