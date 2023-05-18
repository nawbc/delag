export interface DealgListenOptions {
  port?: number | undefined;
  host?: string | undefined;
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
}
