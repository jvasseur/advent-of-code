const range = (start: number, end: number): number[] => [...Array(end - start + 1)].map((el, ind) => ind + start);

export default range;
