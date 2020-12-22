const intersect = <T>(a: T[], b: T[]): T[] => a.filter((value) => b.includes(value));

export default intersect;
