import { z, type ZodType, registry } from "zod";
import {
  buildGroupSchemaFromFields,
  configFieldRegistry,
  configGroupRegistry,
  type MetadataConfigField,
} from "./registries.ts";

export * from "zod";

export { z as s };

export type $SchemaType = ZodType<any>;

const dummyReg = registry();
export const $SchemaRegistry = dummyReg.constructor as any;
export const $ZodRegistry = $SchemaRegistry;

function createConfigProxy<T extends ZodType<any>, M>(
  schema: T | undefined,
  registryInst: any,
  initialMeta?: M,
): any {
  let currentSchema: any = schema;
  const dummyTarget = function () {};
  let proxy: any;

  function registerWithMeta(meta: any) {
    if (!currentSchema) {
      const groupId = meta.id || meta.groupId;
      if (groupId) {
        currentSchema = buildGroupSchemaFromFields(groupId);
      }
    }
    if (currentSchema) {
      registryInst.add(currentSchema, meta);
      if (proxy) {
        registryInst.add(proxy, meta);
      }
    }
    return proxy;
  }

  const handler: ProxyHandler<any> = {
    get(_target, prop, _receiver) {
      if (prop === "meta") {
        return (meta: M) => {
          return registerWithMeta(meta);
        };
      }

      if (prop === "describe") {
        return (description: string) => {
          let meta: any = {};
          if (currentSchema) {
            const existingMeta = registryInst.get(currentSchema);
            if (existingMeta) {
              meta = { ...existingMeta };
            }
          }
          meta.description = description;
          return registerWithMeta(meta);
        };
      }

      if (!currentSchema && prop === "parse") {
        return function (_input: any) {
          throw new Error("Config schema is not initialized or registered with metadata.");
        };
      }

      if (!currentSchema) {
        return undefined;
      }

      const val = Reflect.get(currentSchema, prop, currentSchema);
      if (typeof val === "function") {
        return function (this: any, ...args: any[]) {
          const res = val.apply(currentSchema, args);
          if (res && typeof res === "object" && typeof res.parse === "function") {
            currentSchema = res;
            return proxy;
          }
          return res;
        };
      }
      return val;
    },
  };

  proxy = new Proxy(dummyTarget, handler);

  if (initialMeta) {
    registerWithMeta(initialMeta);
  }

  return proxy;
}

function createConfigWrapper<M>(registryInst: any) {
  function configFn<T extends ZodType<any>>(
    schema: T,
    meta?: M,
  ): T & { meta: (m: M) => T; describe: (desc: string) => T } {
    return createConfigProxy(schema, registryInst, meta);
  }

  return new Proxy(configFn, {
    apply(_target, _thisArg, argArray: [any, any?]) {
      const [schema, meta] = argArray;
      return createConfigProxy(schema, registryInst, meta);
    },
  });
}

/**
 * Proxy for registering a single configuration field schema into `configFieldRegistry`.
 */
export const config = createConfigWrapper<MetadataConfigField>(configFieldRegistry);

/**
 * Proxy for registering a configuration group schema into `configGroupRegistry`.
 * Does not require passing individual field schemas in Schema.object({...}).
 */
export const configGroup = new Proxy(
  function (arg1?: any, arg2?: any) {
    if (arg1 && typeof arg1 === "object" && (arg1.id || arg1.urn || arg1.groupId)) {
      return createConfigProxy(undefined, configGroupRegistry, arg1);
    }
    return createConfigProxy(arg1, configGroupRegistry, arg2);
  },
  {
    apply(_target, _thisArg, argArray: [any?, any?]) {
      const [arg1, arg2] = argArray;
      if (arg1 && typeof arg1 === "object" && (arg1.id || arg1.urn || arg1.groupId)) {
        return createConfigProxy(undefined, configGroupRegistry, arg1);
      }
      return createConfigProxy(arg1, configGroupRegistry, arg2);
    },
  },
) as any;
