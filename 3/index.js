const input = await Deno.readTextFile(new URL('input.txt', import.meta.url));

const grid = input.split('\n').filter((line) => line !== '').map((line) => line.split(''));

const sloppe = (right, down) => {
    let row = 0;
    let col = 0;
    let trees = 0;

    while (row < grid.length) {
        if (grid[row][col % grid[0].length] === '#') {
            trees++;
        }

        row += down;
        col += right;
    }

    return trees;
}

console.log(sloppe(3, 1));
console.log(sloppe(1, 1) * sloppe(3, 1) * sloppe(5, 1) * sloppe(7, 1) * sloppe(1, 2));
