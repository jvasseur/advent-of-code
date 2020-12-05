const input = await Deno.readTextFile('5.txt');

const seats = input.split('\n').filter((line) => line !== '');

const binary = seats.map((seat) => seat.replace(/F/g, '0').replace(/B/g, '1').replace(/L/g, '0').replace(/R/g, '1'));

const seatIds = binary.map((seat) => parseInt(seat, 2));

const maxId = Math.max(...seatIds);
const minId = Math.min(...seatIds);

console.log(maxId);

for (let i = minId; i <= maxId; i++) {
    if (seatIds.includes(i - 1) && seatIds.includes(i + 1) && !seatIds.includes(i)) {
        console.log(i);
    }
}
