export const slowify = function () {
  return function (req: any, res: any, callback: any) {};
};

/**
 * Replace removed middleware with an appropriate error message.
 */

function deprecatedError(name: string): Error {
  return new Error(
    'Most middleware (like ' +
      name +
      ') is no longer bundled with Express and must be installed separately. Please see https://github.com/senchalabs/connect#middleware.',
  );
}

/**
 * Default export and trigger error when using.
 */
export default class {
  static get bodyParser() {
    throw deprecatedError('bodyParser');
  }
  static get compress() {
    throw deprecatedError('compress');
  }
  static get cookieSession() {
    throw deprecatedError('cookieSession');
  }
  static get session() {
    throw deprecatedError('session');
  }
  static get logger() {
    throw deprecatedError('logger');
  }
  static get cookieParser() {
    throw deprecatedError('cookieParser');
  }
  static get favicon() {
    throw deprecatedError('favicon');
  }
  static get responseTime() {
    throw deprecatedError('responseTime');
  }
  static get errorHandler() {
    throw deprecatedError('errorHandler');
  }
  static get timeout() {
    throw deprecatedError('timeout');
  }
  static get methodOverride() {
    throw deprecatedError('methodOverride');
  }
  static get vhost() {
    throw deprecatedError('vhost');
  }
  static get csrf() {
    throw deprecatedError('csrf');
  }
  static get directory() {
    throw deprecatedError('directory');
  }
  static get limit() {
    throw deprecatedError('limit');
  }
  static get multipart() {
    throw deprecatedError('multipart');
  }
  static get staticCache() {
    throw deprecatedError('staticCache');
  }
}
