import { ParsedExpression } from '../parse.ts';
import { Expression } from '../types.ts';

const compile = (parsed: ParsedExpression): Expression => {
    let expression = parsed;

    // Subcompile
    expression = expression.map((item) => Array.isArray(item) ? compile(item) : item);

    // Reduce expression
    while (expression.length > 3) {
        const [left, operator, right, ...rest] = expression;

        expression = [[left, operator, right], ...rest];
    }

    return expression as Expression;
}

export type { Expression };
export default compile;
