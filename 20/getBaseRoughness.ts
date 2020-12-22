import Picture, { pictureSize } from './Picture.ts';

const getBaseRoughness = (image: Picture): number => {
    let roughness = 0;

    for (let x = 0; x < pictureSize; x++) {
        for (let y = 0; y < pictureSize; y++) {
            if (image.get(x, y) === '#') {
                roughness++;
            }
        }
    }

    return roughness;
}

export default getBaseRoughness;
