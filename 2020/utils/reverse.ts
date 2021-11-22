/**
 * Copying reverse.
 */
const reverse = <T>(array: Iterable<T>): T[] => {
    const clone = [...array];

    clone.reverse();

    return clone;
};

export default reverse;
