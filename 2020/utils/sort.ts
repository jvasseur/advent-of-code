/**
 * Copying sort.
 */
const sort = <T>(array: Iterable<T>, compare?: (a: T, b: T) => number): T[] => {
    const clone = [...array];

    clone.sort(compare);

    return clone;
};

export default sort;
