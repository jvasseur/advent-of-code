const isNumeric = (char: string): boolean => /^[0-9]+$/.test(char);

const End = Symbol('End');
type End = typeof End;

type TokenValue = number|string|End;

interface Token {
    value: TokenValue;
    position: number;
}

function* tokenize(expression: string): Generator<Token> {
    for (let i = 0; i < expression.length; i++) {
        const current = expression[i];

        if (current === ' ') {
            continue;
        }

        if (current === '*' || current === '+' || current === '(' || current === ')') {
            yield { value: current, position: i } ;

            continue;
        }

        if (isNumeric(current)) {
            let number = current;

            while (i < expression.length && isNumeric(expression[i + 1])) {
                i++;

                number += expression[i + 1];
            }

            yield { value: parseInt(number, 10), position: i };

            continue;
        }

        throw new Error(`Invalid character: ${current}`)
    }

    yield {
        value: End,
        position: expression.length,
    }
};

export type { Token, TokenValue };
export { End };
export default tokenize;
