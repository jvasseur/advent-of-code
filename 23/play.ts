import reduceAll from '../utils/reduceAll.ts';
import reverse from '../utils/reverse.ts';

const getDestination = (current: number, max: number): number => current > 1 ? current - 1 : max;

class Cup {
    value: number;
    next: Cup;

    constructor(value: number, next: Cup|null) {
        this.value = value;
        this.next = next as Cup;
    }

    static fromArray(array: number[]): Cup {
        const [first, ...rest] = reverse(array);

        const firstCup = new Cup(first, null);

        let list = firstCup;

        for (const element of rest) {
            list = new Cup(element, list);
        }

        firstCup.next = list;

        return list;
    }

    toArray(): number[] {
        const array: number[] = [];

        forEachCup((cup) => array.push(cup.value), this);

        return array;
    }
}

const forEachCup = (callback: (cup: Cup) => void, start: Cup): void => {
    let current: Cup = start;
    do {
        callback(current);

        current = current.next;
    } while (current !== start)
}

const play = (cups: number[], rounds: number): number[] => {
    const max = reduceAll((a, b) => Math.max(a, b), cups);

    let current = Cup.fromArray(cups);

    const cupsByNumber: Cup[] = [...Array(cups.length)];

    forEachCup((cup) => cupsByNumber[cup.value] = cup, current);

    for (let i = 0; i < rounds; i++) {
        const picked = current.next;

        let destination = getDestination(current.value, max);
        while (destination === picked.value || destination === picked.next.value || destination === picked.next.next.value) {
            destination = getDestination(destination, max);
        }

        const index = cupsByNumber[destination];

        current.next = picked.next.next.next;
        picked.next.next.next = index.next;
        index.next = picked;

        current = current.next;
    }

    return current.toArray();
}

export default play;
