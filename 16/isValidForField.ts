import { Field } from './types.ts';

const isValidForField = (value: number, { ranges }: Field) => ranges.some(({ min, max }) => value >= min && value <= max);

export default isValidForField;
