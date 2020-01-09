/* tslint:disable */
/* eslint-disable */
export class Server {
  static load(): Promise<Server>;

  set(uri: string, code: string): void;

  evaluate(uri: string): string | undefined;
}
