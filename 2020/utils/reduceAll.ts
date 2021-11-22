const reduceAll = <T>(
    callback: (previousValue: T, currentValue: T, currentIndex: number) => T,
    [first, ...rest]: T[],
): T => rest.reduce(callback, first);

export default reduceAll;
