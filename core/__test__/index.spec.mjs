// import test from 'ava';
import * as rs from '../index.js';
import { EventEmitter } from 'events';
// import { } from 'event';

// test('sum from native', (t) => {
// });

console.log(rs);
rs.startServer((err, req, res) => {
  // console.log(req);

  // req.callback('1231321');

  // await new Promise((resolve, reject) => {
  //   setTimeout(resolve, 5000);
  // });

  return '1111';

  // rs.response({
  //   data: 'demo',
  //   headers: {
  //     Cookie: 'Demo',
  //   },
  // });
});

// class Server extends EventEmitter {
//   constructor(options, callback) {
//     this.on('request', callback);
//   }
// }
