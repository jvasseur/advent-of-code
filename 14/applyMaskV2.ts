const replace = (
    string: string,
    index: number,
    replacement: string,
): string => string.substr(0, index) + replacement + string.substr(index + 1);

const applyMaskV2 = (index: number, mask: string): number[] => {
    let indexes = [index.toString(2).padStart(36, '0')];

    for (let i = 0; i < 36; i++) {
        switch (mask[i]) {
            case '0':
                // no-op
                break;
            case '1':
                indexes = indexes.map((binary) => replace(binary, i, '1'));
                break;
            case 'X':
                indexes = [
                    ...indexes.map((binary) => replace(binary, i, '0')),
                    ...indexes.map((binary) => replace(binary, i, '1')),
                ];
                break;
        }
    }

    return indexes.map((index) => parseInt(index, 2));
}

export default applyMaskV2;
