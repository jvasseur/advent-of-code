enum Rotation {
    Rotate90,
    Rotate180,
    Rotate270,
}

enum Flip {
    VerticalFlip,
    HorizontalFlip,
}

const rotations = [null, Rotation.Rotate90, Rotation.Rotate180, Rotation.Rotate270];
const flips = [null, Flip.VerticalFlip, Flip.HorizontalFlip];

export { Rotation, rotations, Flip, flips };
