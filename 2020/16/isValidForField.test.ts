import { assertEquals } from 'https://deno.land/std@0.80.0/testing/asserts.ts';
import isValidForField from './isValidForField.ts';

Deno.test('It validates values properly', () => {
    assertEquals(isValidForField(3, {
        name: 'row',
        ranges: [
            { min: 0, max: 5},
            { min: 8, max: 19},
        ],
    }), true);
});
