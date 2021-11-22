import tokenize, { End, Token, TokenValue } from './tokenize.ts';

type ParsedExpression = (number|string|ParsedExpression)[];

function* parseIteration(tokens: Iterator<Token>, end: TokenValue): Generator<number|string|ParsedExpression> {
    while (true) {
        let { done, value: token } = tokens.next();

        if (done) {
            throw new Error('Unexpected end of expression');
        }

        if (token.value === end) {
            return;
        }

        if (token.value === '(') {
            yield parseUntil(tokens, ')');

            continue;
        }

        yield token.value;
    }
};

const parseUntil = (tokens: Iterator<Token>, end: TokenValue): ParsedExpression => Array.from(parseIteration(tokens, end));

const parse = (expression: string): ParsedExpression => {
    const tokens = tokenize(expression);

    return parseUntil(tokens, End);
};

export type { ParsedExpression };
export default parse;
