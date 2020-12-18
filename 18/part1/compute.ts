import compile from './compile.ts';
import execute from '../execute.ts';
import parse from '../parse.ts';

const compute = (expression: string) => execute(compile(parse(expression)));

export default compute;
