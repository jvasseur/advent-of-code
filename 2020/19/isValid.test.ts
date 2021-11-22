import { assertEquals } from 'https://deno.land/std@0.80.0/testing/asserts.ts';
import isValid from './isValid.ts';

Deno.test('First example', () => {
    const rules = [
        [[1, 2]],
        'a',
        [[1, 3], [3, 1]],
        'b',
    ];

    assertEquals(isValid(rules, 0, 'aab'), true);
    assertEquals(isValid(rules, 0, 'aba'), true);

    assertEquals(isValid(rules, 0, 'aaa'), false);

    assertEquals(isValid(rules, 0, 'aa'), false);
    assertEquals(isValid(rules, 0, 'aaba'), false);
});

Deno.test('Second example', () => {
    const rules = [
        [[4, 1, 5]],
        [[2, 3], [3, 2]],
        [[4, 4], [5, 5]],
        [[4, 5], [5, 4]],
        'a',
        'b',
    ];

    assertEquals(isValid(rules, 0, 'aaaabb'), true);
    assertEquals(isValid(rules, 0, 'aaabab'), true);
    assertEquals(isValid(rules, 0, 'abbabb'), true);
    assertEquals(isValid(rules, 0, 'abbbab'), true);
    assertEquals(isValid(rules, 0, 'aabaab'), true);
    assertEquals(isValid(rules, 0, 'aabbbb'), true);
    assertEquals(isValid(rules, 0, 'abaaab'), true);
    assertEquals(isValid(rules, 0, 'ababbb'), true);

    assertEquals(isValid(rules, 0, 'aaaaaa'), false);

    assertEquals(isValid(rules, 0, 'aaaab'), false);
    assertEquals(isValid(rules, 0, 'aaaabba'), false);
});

Deno.test('Loop', () => {
    const rules = [
        [[1], [1, 0]],
        'a',
    ];

    assertEquals(isValid(rules, 0, 'a'), true, 'a');
    assertEquals(isValid(rules, 0, 'aa'), true, 'aa');
    assertEquals(isValid(rules, 0, 'aaa'), true, 'aaa');
    assertEquals(isValid(rules, 0, 'aaaa'), true, 'aaaa');

    assertEquals(isValid(rules, 0, 'aaaab'), false, 'aaaab');
});
