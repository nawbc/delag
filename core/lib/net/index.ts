import { debuglog } from 'node:util';
import { is } from '../gadget';
import { DelagListenOptions } from './net_interface';
import EventEmitter from 'node:events';
import * as RsDelag from '../../index';
import * as NodeNet from 'node:net';
import * as https from 'node:https';
https.createServer();

let a = new NodeNet.SocketAddress({});
let debug = debuglog('delag:net');

class Socket extends EventEmitter {
  public connecting = false;
}

export class Server extends EventEmitter {
  #server;

  static readonly DEFAULT_IPV4_ADDR = '0.0.0.0';
  static readonly DEFAULT_IPV6_ADDR = '::';
  static readonly DEFAULT_PORT = 80;

  constructor(options, callback) {
    super();
  }

  // Copyright Joyent, Inc. and other Node contributors.
  #normalizeParams(p: any): [DelagListenOptions, (() => void) | null] {
    let options: DelagListenOptions = {
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

  /**
   * @todo Support listen in cluster.
   */
  #clusterListen() {}

  /**
   * Start a server listening for connections.
   *
   * Support node:cluster.
   *
   * @param port
   * @param hostname
   * @param backlog
   * @param listener
   */
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
  public listen(options: DelagListenOptions, listener?: () => void): this;
  public listen(...params): this {
    const [options, callback] = this.#normalizeParams(params);

    const { port, host } = options as Pick<
      Required<DelagListenOptions>,
      'host' | 'port'
    >;

    try {
      this.#server = RsDelag.serve(
        {
          port,
          host,
        },
        (err, req) => {
          this.emit('request', req);

          return {
            body: 'demo',
          };
        },
      );
    } catch (error) {
      this.emit('clientError', error);
    }

    return this;
  }

  public close() {}
}
