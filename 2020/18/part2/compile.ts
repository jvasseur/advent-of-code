import { ParsedExpression } from '../parse.ts';
import { Expression, Operation } from '../types.ts';

const compile = (parsed: ParsedExpression): Expression => {
    let expression = parsed;

    // Subcompile
    expression = expression.map((item) => Array.isArray(item) ? compile(item) : item);

    if (expression.length === 1) {
        return expression[0] as number;
    }

    if (expression.length === 3) {
        return expression as Operation;
    }

    // Group additions
    for (let i = 1; i < expression.length; i += 2) {
        if (expression[i] === '+') {
            return compile([
                ...expression.slice(0, i - 1),
                [expression[i - 1], '+', expression[i + 1]],
                ...expression.slice(i + 2),
            ]);
        }
    }

    // Reduce expression
    while (expression.length > 3) {
        const [left, operator, right, ...rest] = expression;

        expression = [[left, operator, right], ...rest];
    }

    return expression as Expression;
}

export type { Expression };
export default compile;
