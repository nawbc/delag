// import test from 'ava';
import * as rs from '../index.js';
import { EventEmitter } from 'events';
// import { } from 'event';

// test('sum from native', (t) => {
// });

console.log(rs);
rs.createServer((err, req, res) => {
  console.log(req);
});

// class Server extends EventEmitter {
//   constructor(options, callback) {
//     this.on('request', callback);
//   }
// }
