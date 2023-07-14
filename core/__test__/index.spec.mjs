// import { Server } from '../dist';
// const { Server } = require('../dist');

// const server = new Server();

// server.on('request', (req) => {
//   console.log(req, '1111');
// });

// server.listen(8080, '127.0.0.1');

// import test from 'ava';
import { Socket } from 'net';
import * as Delag from '../index.js';
import { EventEmitter } from 'events';
import { Readable, Writable } from 'stream';

// let a = Delag.callbackArgsWithCallEmit(async (v) => {
//   console.debug(v);
//   return new Promise((r) => setTimeout(() => r(100), 3000));
// });

// let b = await a;

// console.debug(b);

// import { ReadableStream, WritableStream } from 'node:stream/web';

// class Response extends Write {}

// const server = createServer();

// let d = server.listen(0, (err) => {});

// console.log(cluster.isMaster);
// if (cluster.isMaster) {
//   for (var i = 0; i < os.cpus().length; i++) {
//     cluster.fork();
//   }
// } else {
//   // http
//   //   .createServer(function (req, res) {
//   //     res.writeHead(200);
//   //     res.end(' says hello!');
//   //   })
//   //   .listen(8080);
// }

console.log(process.pid);
console.log(process.ppid);
Delag.serve({ port: 8080, host: '127.0.0.1' }, (err, req) => {
  // console.log(req);

  if (err) {
    console.log(err, '======');
  }

  let fd = Number(req.fd);
  let socket = new Socket({
    fd,
    readable: true,
    writable: true,
  });

  // console.log(socket);
  // console.log(req);

  // const emitter = new EventEmitter();

  // emitter.on('data', (data) => {
  //   // console.log(data.toString('utf8'));
  // });

  // req._bodyCallEmit(emitter.emit.bind(emitter));

  // let a = req.body((...b) => {
  //   console.log(b);
  // });

  // console.log(a);

  // let a = req.body((...a) => {
  //   console.log('================');
  //   console.log(a);
  // });
  // console.log(a);

  return {
    body: 'Hello, World',
    headers: {
      Cookie: 'xxx=xxxx',
    },
  };
});

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
