import reduceAll from '../utils/reduceAll.ts';

const gcd = (a, b) => {
    while (b != 0) {
       [a, b] = [b, a % b];
    }

    return a;
}

const lcm = (a, b) => (a * b) / gcd(a, b);

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

    const result = reduceAll((previous, current) => {
        let timestamp = previous[0];
        while ((timestamp + current[0]) % current[1] !== 0) {
            timestamp += previous[1];
        }

        return [timestamp, lcm(previous[1], current[1])];
    }, availableBuses);

    console.log(result[0]);
}
