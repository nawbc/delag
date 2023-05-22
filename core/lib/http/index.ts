// Copyright Han, Deskbtm. and other Delag contributors.
// SPDX-License-Identifier: Apache-2.0

import { EventEmitter } from 'events';
import * as DelagCore from '../../index';
import { Readable, Writable } from 'stream';
import { DealgListenOptions } from './http.interface';
import { is } from '../gadget';
import { debuglog } from 'node:util';

let debug = debuglog('delag:http');

class IncomingMessage extends Readable {}

class OutgoingMessage extends Writable {}

export class Server extends EventEmitter {
  private _server;

  static DEFAULT_IPV4_ADDR = '0.0.0.0';
  static DEFAULT_IPV6_ADDR = '::';
  static DEFAULT_PORT = 80;

  constructor(options, callback) {
    super();

    this.on('request', callback);
  }

  private normalizeParams(p: any): [DealgListenOptions, (() => void) | null] {
    let options: DealgListenOptions = {
      port: Server.DEFAULT_PORT,
      host: Server.DEFAULT_IPV6_ADDR,
    };

    if (void 0 == p) {
      return [options, null];
    }

    const p0 = p[0];

    if (is.object(p0)) {
      options = Object.assign({}, options, p0);
    } else if (is.string(p0)) {
      options.path = p0;
    } else {
      const p1 = p[1];
      options.port = p0;
      if (p.length > 1 && is.string(p1)) {
        options.host = p1;
      }
    }

    const cb = p[p.length - 1];

    let arr;

    if (!is.function(cb)) {
      arr = [options, null];
    } else {
      arr = [options, cb];
    }

    return arr;
  }

  public address;

  public listen(
    port?: number,
    hostname?: string,
    backlog?: number,
    listener?: () => void,
  ): this;
  public listen(port?: number, hostname?: string, listener?: () => void): this;
  public listen(port?: number, backlog?: number, listener?: () => void): this;
  public listen(port?: number, listener?: () => void): this;
  public listen(path: string, backlog?: number, listener?: () => void): this;
  public listen(path: string, listener?: () => void): this;
  public listen(options: DealgListenOptions, listener?: () => void): this;
  public listen(...params): this {
    const [options, callback] = this.normalizeParams(params);
    const { port, host } = options as Pick<
      Required<DealgListenOptions>,
      'host' | 'port'
    >;

    try {
      this._server = DelagCore.serve(
        {
          port,
          host,
        },
        (req) => {},
      );
    } catch (error) {
      this.emit('clientError');
    }

    return this;
  }
}
