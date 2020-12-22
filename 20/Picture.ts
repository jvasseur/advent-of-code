import { Rotation, rotations, Flip, flips } from './transformations.ts';

const pictureSize = 12 * 8;

interface Picture {
    get(x: number, y: number): string
}

class RotatedPicture implements Picture {
    #parent: Picture;
    #rotation: Rotation;

    constructor(parent: Picture, roation: Rotation) {
        this.#parent = parent;
        this.#rotation = roation;
    }

    get(x: number, y: number): string {
        switch (this.#rotation) {
            case Rotation.Rotate90:
                return this.#parent.get(y, pictureSize - 1 - x);
            case Rotation.Rotate180:
                return this.#parent.get(pictureSize - 1 - x, pictureSize - 1 - y);
            case Rotation.Rotate270:
                return this.#parent.get(pictureSize - 1 - y, x);
        }
    }
}

class FlippedPicture implements Picture {
    #parent: Picture;
    #flip: Flip;

    constructor(parent: Picture, flip: Flip) {
        this.#parent = parent;
        this.#flip = flip;
    }

    get(x: number, y: number): string {
        switch (this.#flip) {
            case Flip.VerticalFlip:
                return this.#parent.get(x, pictureSize - 1 - y);
            case Flip.HorizontalFlip:
                return this.#parent.get(pictureSize - 1 - x, y);
        }
    }
}

function* transformPicture(picture: Picture): Generator<Picture> {
    for (const rotation of rotations) {
        const rotatedPicture = rotation !== null ? new RotatedPicture(picture, rotation) : picture;

        for (const flip of flips) {
            yield flip !== null ? new FlippedPicture(rotatedPicture, flip) : rotatedPicture;
        }
    }
}

export { pictureSize, transformPicture };
export default Picture;
