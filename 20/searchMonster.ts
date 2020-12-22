import Picture, { pictureSize } from './Picture.ts';
import isMonster, { monsterSize } from './isMonster.ts';

const searchMonster = (image: Picture): number => {
    let monsters = 0;

    for (let x = 0; x < pictureSize - monsterSize[0]; x++) {
        for (let y = 0; y < pictureSize - monsterSize[1]; y++) {
            if (isMonster(image, x, y)) {
                monsters++;
            }
        }
    }

    return monsters;
}

export default searchMonster;
