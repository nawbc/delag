// import test from 'ava';
import * as Delag from '../index.js';
import { EventEmitter } from 'events';
import { Readable, Writable } from 'stream';

// class Response extends Write {}

Delag.serve((err, req) => {
  let r = req;

  // console.debug(r);

  return {
    data: 'Hello, World',
    headers: {
      Cookie: 'xxx=xxxx',
    },
  };
});

class IncomingMessage extends Readable {}

class OutgoingMessage extends Writable {}

class Server extends EventEmitter {
  constructor(options, callback) {
    super();

    this.on('request', callback);
  }

  listen() {}
}
