import grid from '../utils/grid.ts';
import Picture from './Picture.ts';

const monsterText = `                  # \n#    ##    ##    ###\n #  #  #  #  #  #   `;

const monsterSize: [number, number] = [20, 3];

const monsterData = monsterText.split('\n').map((line) => line.split(''));

const monster = grid(...monsterSize, '');

for (let x = 0; x < monsterSize[0]; x++) {
    for (let y = 0; y < monsterSize[1]; y++) {
        monster[x][y] = monsterData[2 - y][x]
    }
}

const isMonster = (image: Picture, searchX: number, searchY: number): boolean => {
    for (let x = 0; x < monsterSize[0]; x++) {
        for (let y = 0; y < monsterSize[1]; y++) {
            if (monster[x][y] === ' ') {
                continue;
            }

            if (image.get(searchX + x, searchY + y) !== '#') {
                return false;
            }
        }
    }

    return true;
}

export { monsterSize };
export default isMonster;
