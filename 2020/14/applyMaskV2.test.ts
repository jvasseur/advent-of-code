import { assertEquals } from 'https://deno.land/std@0.80.0/testing/asserts.ts';
import applyMaskV2 from './applyMaskV2.ts';

Deno.test('example', () => {
    assertEquals(
        applyMaskV2(42, '000000000000000000000000000000X1001X').sort((a, b) => a - b),
        [26, 27, 58, 59],
    );
    assertEquals(
        applyMaskV2(26, '00000000000000000000000000000000X0XX').sort((a, b) => a - b),
        [16, 17, 18, 19, 24, 25, 26, 27],
    );
});
