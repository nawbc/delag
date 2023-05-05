import { EventEmitter } from 'events';
import * as rs from '../index';

export interface ServerOptions {}

class Server extends EventEmitter {
  constructor(options, callback) {
    super();

    this.on('request', callback);
  }

  public listen() {}
}
