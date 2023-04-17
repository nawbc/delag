// import test from 'ava';
import { createServer } from '../index.js';

// test('sum from native', (t) => {
// });

createServer((...args) => {
  console.log('===================');
  console.log(args);
});
