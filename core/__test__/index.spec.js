// import { Server } from '../dist';
const { Server } = require('../dist');

const server = new Server();

server.on('request', (req) => {
  console.log(req, '1111');
});

server.listen(8080, '127.0.0.1');

// // import test from 'ava';
// import * as Delag from '../index.js';
// import { EventEmitter } from 'events';
// import { Readable, Writable } from 'stream';

// // import { ReadableStream, WritableStream } from 'node:stream/web';

// // class Response extends Write {}

// // const server = createServer();

// // let d = server.listen(0, (err) => {});

// // console.log(cluster.isMaster);
// // if (cluster.isMaster) {
// //   for (var i = 0; i < os.cpus().length; i++) {
// //     cluster.fork();
// //   }
// // } else {
// //   // http
// //   //   .createServer(function (req, res) {
// //   //     res.writeHead(200);
// //   //     res.end(' says hello!');
// //   //   })
// //   //   .listen(8080);
// // }
// Delag.serve({ port: 8080, host: '127.0.0.1' }, (err, req) => {
//   // console.log(req);
//   return {
//     body: 'Hello, World',
//     headers: {
//       Cookie: 'xxx=xxxx',
//     },
//   };
// });

// class IncomingMessage extends Readable {
//   constructor(req, opts) {}
// }

// class OutgoingMessage extends Writable {}

// class Server extends EventEmitter {
//   constructor(options, callback) {
//     super();

//     this.on('request', callback);
//   }

//   //   listen(port: number, hostname: string, backlog: number, callback?: () => void): http.Server;
//   // listen(port: number, hostname: string, callback?: () => void): http.Server;
//   // listen(port: number, callback?: () => void): http.Server;
//   // listen(callback?: () => void): http.Server;
//   // listen(path: string, callback?: () => void): http.Server;
//   // listen(handle: any, listeningListener?: () => void): http.Server;
//   listen() {
//     Delag.serve((err, req) => {
//       let r = req;
//       this.emit('', req);

//       // return {
//       //   data: 'Hello, World',
//       //   headers: {
//       //     Cookie: 'xxx=xxxx',
//       //   },
//       // };
//     });
//   }
// }
