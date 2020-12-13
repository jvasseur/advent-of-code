import reduceAll from '../utils/reduceAll.js';

const gcd = (a, b) => {
    let t;
    while (b != 0) {
       t = b;
       b = a % b;
       a = t;
    }

    return a;
}

const lcm = (n1, n2) => (n1 * n2) / gcd(n1, n2);

const [timestampInput, busesInput] = (await Deno.readTextFile('input.txt')).split('\n');

const timestamp = parseInt(timestampInput, 10);
const buses = busesInput.split(',').map((busInput) => busInput === 'x' ? busInput : parseInt(busInput, 10));

{
    const availableBuses = buses.filter((bus) => bus !== 'x');

    const nextBuses = availableBuses.map((bus) => (Math.floor(timestamp / bus) + 1) * bus);

    const nextBusIndex = nextBuses.indexOf(Math.min(...nextBuses));

    const nextBus = availableBuses[nextBusIndex];
    const nextBusTime = nextBuses[nextBusIndex];

    console.log(nextBus * (nextBusTime - timestamp));
}

{
    const availableBuses = [...buses.entries()].filter(([key, value]) => value !== 'x');

    const result = reduceAll((previous, curent) => {
        let timestamp = previous[0];
        while ((timestamp + curent[0]) % curent[1] !== 0) {
            timestamp += previous[1];
        }

        return [timestamp, lcm(previous[1], curent[1])];
    }, availableBuses);

    console.log(result[0]);
}
