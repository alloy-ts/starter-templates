import type { ZodType, z } from "zod";

export declare const $output: unique symbol;
export type $output = typeof $output;
export declare const $input: unique symbol;
export type $input = typeof $input;

export type $SchemaType = ZodType<any>;

export type $replace<Meta, S extends $SchemaType> = Meta extends $output
  ? any
  : Meta extends $input
    ? any
    : Meta extends (infer M)[]
      ? $replace<M, S>[]
      : Meta extends (...args: infer P) => infer R
        ? (...args: { [K in keyof P]: $replace<P[K], S> }) => $replace<R, S>
        : Meta extends object
          ? { [K in keyof Meta]: $replace<Meta[K], S> }
          : Meta;

export type MetadataType = object | undefined;

export declare class $SchemaRegistry<
  Meta extends MetadataType = MetadataType,
  Schema extends $SchemaType = $SchemaType,
> implements Iterable<any> {
  _meta: Meta;
  _schema: Schema;
  _map: WeakMap<Schema, $replace<Meta, Schema>>;
  _idmap: Map<string, Schema>;
  get size(): number;
  add<S extends Schema>(
    schema: S,
    ..._meta: undefined extends Meta ? [$replace<Meta, S>?] : [$replace<Meta, S>]
  ): this;
  clear(): this;
  remove(schema: Schema | string): this;
  get<S extends Schema>(
    schema: S | string,
  ): $replace<Meta, S> | { schema: Schema; meta: $replace<Meta, S> } | undefined;
  has(schema: Schema | string): boolean;
  [Symbol.iterator](): Iterator<any>;
}

export declare class $ZodRegistry<
  Meta extends MetadataType = MetadataType,
  Schema extends $SchemaType = $SchemaType,
> extends $SchemaRegistry<Meta, Schema> {}

export interface JSONSchemaMeta {
  id?: string | undefined;
  title?: string | undefined;
  description?: string | undefined;
  deprecated?: boolean | undefined;
  [k: string]: unknown;
}

export interface GlobalMeta extends JSONSchemaMeta {}

export declare function registry<
  T extends MetadataType = MetadataType,
  S extends $SchemaType = $SchemaType,
>(): $SchemaRegistry<T, S>;

export declare const globalRegistry: $SchemaRegistry<GlobalMeta>;

export declare const s: typeof z;

export declare function config<T extends $SchemaType>(
  schema: T,
  meta?: any,
): T & {
  meta: (m: any) => T;
  describe: (desc: string) => T;
};

export declare function configGroup(schemaOrMeta?: any, meta?: any): any;

export * from "zod";
