const applyMask = (value: number, mask: string): number => {
    const binary = value.toString(2).padStart(36, '0').split('');

    for (let i = 0; i < 36; i++) {
        if (mask[i] !== 'X') {
            binary[i] = mask[i];
        }
    }

    return parseInt(binary.join(''), 2);
}

export default applyMask;
