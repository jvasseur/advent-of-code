import { Expression } from './types.ts';

const execute = (expression: Expression): number => {
    if (typeof expression === 'number') {
        return expression;
    }

    const [left, operator, right] = expression;

    switch (operator) {
        case '+':
            return execute(left) + execute(right);
        case '*':
            return execute(left) * execute(right);
    }
}

export default execute;
