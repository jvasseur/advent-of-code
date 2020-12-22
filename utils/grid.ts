const grid = <T>(sizeX: number, sizeY: number, fill: T): T[][] => [...Array(sizeX)].map(() => [...Array(sizeY)].map(() => fill));

export default grid;
