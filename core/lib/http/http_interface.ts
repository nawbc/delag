export interface DealgListenOptions {
  port?: number | undefined;
  host?: string | undefined;

  /**
   * the length of the listening queue
   *
   * @default 2048
   */
  backlog?: number | undefined;
  /**
   * @todo https://nodejs.org/docs/latest/api/net.html#identifying-paths-for-ipc-connections
   */
  path?: string | undefined;
  // exclusive?: boolean | undefined;
  // readableAll?: boolean | undefined;
  // writableAll?: boolean | undefined;
  /**
   * @todo
   * @default false
   */
  ipv6Only?: boolean | undefined;

  // Extra properties.

  /**
   * Set number of workers to start. Number must be greater than 0.
   * The default worker count is the number of physical CPU cores available.
   *
   * @default os.cpus().length
   */
  workers?: number;

  /**
   * Set max number of threads for each worker's blocking task thread pool.
   * One thread pool is set up **per worker**; not shared across workers.
   *
   * @example
   * ```
   *  {
   *    workers: 4  // server has 4 worker thread.
   *    workerMaxBlockingThreads: 4 // every worker has 4 max blocking threads.
   * }
   *
   * ```
   *
   */
  workerMaxBlockingThreads?: number;
}
