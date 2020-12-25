import parseInt10 from '../utils/parseInt10.ts';
import findLoopSize from './findLoopSize.ts';
import transform from './transform.ts';

const input = await Deno.readTextFile(new URL('input.txt', import.meta.url));

const [doorKey, cardKey] = input.split('\n').map(parseInt10);

const doorLoopSize = findLoopSize(doorKey);

const encryptionKey = transform(cardKey, doorLoopSize);

console.log(encryptionKey);
