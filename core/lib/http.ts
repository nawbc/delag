import { EventEmitter } from 'events';
import * as rs from '../index';
import { Readable } from 'stream';

export interface ServerOptions {}

class IncomingMessage extends Readable {

  

}

class Server extends EventEmitter {
  constructor(options, callback) {
    super();

    this.on('request', callback);
  }

  public listen() {}
}

// var a: typeof = Demo32;

// const Demo1: Demo = function (a: number) {};

// class Demo2 {
//   demo() {}
// }

// type Demo22 = Demo2;

// var b: A = [1, '];

// var a: typeof Demo2 = Demo2;

// var a = new Demo2();
// function create(clz: Demo) {
//   return new clz(10);
// }

// function Demo1(a: number) {}

// const a = create(Demo1);
