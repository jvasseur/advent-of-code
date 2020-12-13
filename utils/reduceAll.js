const reduceAll = (callback, [first, ...rest]) => rest.reduce(callback, first);

export default reduceAll;
