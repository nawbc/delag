// import test from 'ava';
import { createServer } from '../index.js';

// test('sum from native', (t) => {
// });

console.log(createServer);

createServer((...args) => {
  console.log(args);
});
