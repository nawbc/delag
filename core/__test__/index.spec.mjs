// import test from 'ava';
import * as Delag from '../index.js';
import { EventEmitter } from 'events';
import { Readable, Writable } from 'stream';
import worker from 'worker_threads';
import { createServer } from 'node:http';
// import { ReadableStream, WritableStream } from 'node:stream/web';

// class Response extends Write {}

const server = createServer();

let d = server.listen(0, () => {
  
});

Delag.serve((err, req) => {
  return {
    body: 'Hello, World',
    headers: {
      Cookie: 'xxx=xxxx',
    },
  };
});

class IncomingMessage extends Readable {
  constructor(req, opts) {}
}

class OutgoingMessage extends Writable {}

class Server extends EventEmitter {
  constructor(options, callback) {
    super();

    this.on('request', callback);
  }

  //   listen(port: number, hostname: string, backlog: number, callback?: () => void): http.Server;
  // listen(port: number, hostname: string, callback?: () => void): http.Server;
  // listen(port: number, callback?: () => void): http.Server;
  // listen(callback?: () => void): http.Server;
  // listen(path: string, callback?: () => void): http.Server;
  // listen(handle: any, listeningListener?: () => void): http.Server;
  listen() {
    Delag.serve((err, req) => {
      let r = req;
      this.emit('', req);

      // return {
      //   data: 'Hello, World',
      //   headers: {
      //     Cookie: 'xxx=xxxx',
      //   },
      // };
    });
  }
}
