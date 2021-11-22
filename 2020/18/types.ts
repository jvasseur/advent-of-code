type Operator = '+'|'*';

type Operation = [Expression, Operator, Expression];

type Expression = number|Operation;

export type { Expression, Operation };
