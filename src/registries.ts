import { object, registry, type ZodType } from "zod";

/** Metadata base for all registries. */
export type MetadataBase = {
  urn: string;
  title?: string;
  description?: string;
  moduleUrl?: string;
  hash?: string;
  deprecated?: boolean;
  examples?: any[];
};

export type MetadataConfigField = {
  key: string;
  groupId: string;
} & MetadataBase;

export type MetadataConfigGroup = {
  id: string;
  resolveMap?: {
    system?: string | string[];
    user?: string | string[];
    local?: string | string[];
  };
} & MetadataBase;

/**
 * Configuration field registry.
 * Configuration fields are key-value pairs within a {@link ZodType}.
 */
export const configFieldRegistry = registry<MetadataConfigField, ZodType<any>>();

export const configFieldReg = configFieldRegistry;

/**
 * Configuration group registry.
 * Configuration groups are collections of configuration fields sharing the same grouping.
 */
export const configGroupRegistry = registry<MetadataConfigGroup, ZodType<any>>();

export const configGroupReg = configGroupRegistry;

const registeredFieldsMap = new Map<string, MetadataConfigField>();
export const fieldsByGroupIdMap = new Map<
  string,
  Map<string, { schema: any; meta: MetadataConfigField }>
>();
export const registeredGroupsMap = new Map<string, { schema: any; meta: MetadataConfigGroup }>();

/** Enhance registry method implementations for Map / WeakMap / Key string lookup */
function wrapRegistry(reg: any, mapType: "field" | "group") {
  const origGet = reg.get.bind(reg);
  const origHas = reg.has.bind(reg);
  const origRemove = reg.remove.bind(reg);
  const origClear = reg.clear ? reg.clear.bind(reg) : undefined;

  Object.defineProperty(reg, "size", {
    get() {
      if (mapType === "field") {
        return registeredFieldsMap.size;
      }
      return registeredGroupsMap.size;
    },
    configurable: true,
  });

  reg.get = function (schemaOrKey: any) {
    if (typeof schemaOrKey === "string") {
      if (mapType === "field") {
        const found = registeredFieldsMap.get(schemaOrKey);
        if (found) return found;
        for (const fieldMap of fieldsByGroupIdMap.values()) {
          for (const [k, v] of fieldMap.entries()) {
            if (k === schemaOrKey || v.meta.urn === schemaOrKey) return v.meta;
          }
        }
        return undefined;
      } else {
        const found = registeredGroupsMap.get(schemaOrKey);
        if (found) return found.meta;
        for (const entry of registeredGroupsMap.values()) {
          if (entry.meta.urn === schemaOrKey) return entry.meta;
        }
        return undefined;
      }
    }
    return origGet(schemaOrKey);
  };

  reg.has = function (schemaOrKey: any) {
    if (typeof schemaOrKey === "string") {
      return reg.get(schemaOrKey) !== undefined;
    }
    return origHas(schemaOrKey);
  };

  reg.remove = function (schemaOrKey: any) {
    if (typeof schemaOrKey === "string") {
      if (mapType === "field") {
        registeredFieldsMap.delete(schemaOrKey);
      } else {
        registeredGroupsMap.delete(schemaOrKey);
      }
    }
    return origRemove(schemaOrKey);
  };

  if (origClear) {
    reg.clear = function () {
      if (mapType === "field") {
        registeredFieldsMap.clear();
        fieldsByGroupIdMap.clear();
      } else {
        registeredGroupsMap.clear();
      }
      return origClear();
    };
  }
}

wrapRegistry(configFieldRegistry, "field");
wrapRegistry(configGroupRegistry, "group");

// Make configFieldRegistry iterable
(configFieldRegistry as any)[Symbol.iterator] = function* () {
  for (const groupFields of fieldsByGroupIdMap.values()) {
    yield* groupFields.values();
  }
};

// Make configGroupRegistry iterable
(configGroupRegistry as any)[Symbol.iterator] = function* () {
  yield* registeredGroupsMap.values();
};

const origFieldAdd = configFieldRegistry.add.bind(configFieldRegistry);
configFieldRegistry.add = function (schema: any, meta: any) {
  if (meta && typeof meta === "object" && meta.groupId && meta.key) {
    registeredFieldsMap.set(`${meta.groupId}:${meta.key}`, meta);

    let groupFields = fieldsByGroupIdMap.get(meta.groupId);
    if (!groupFields) {
      groupFields = new Map();
      fieldsByGroupIdMap.set(meta.groupId, groupFields);
    }
    groupFields.set(meta.key, { schema, meta });
  }
  return origFieldAdd(schema, meta);
};

export function isFieldRegistered(groupId: string, fieldKey: string): boolean {
  return registeredFieldsMap.has(`${groupId}:${fieldKey}`);
}

export function getFieldsForGroupId(
  groupId: string,
): Map<string, { schema: any; meta: MetadataConfigField }> {
  return fieldsByGroupIdMap.get(groupId) || new Map();
}

/**
 * Dynamically builds a Zod object schema for a group ID from all registered fields in configFieldRegistry.
 */
export function buildGroupSchemaFromFields(groupId: string): ZodType<any> {
  const fields = getFieldsForGroupId(groupId);
  const shape: Record<string, any> = {};

  for (const [key, fieldEntry] of fields.entries()) {
    shape[key] = fieldEntry.schema.optional();
  }

  return object(shape).passthrough().readonly();
}

export function validateGroupFields(groupSchema: any, groupId: string) {
  let shape = groupSchema?.shape || groupSchema?._def?.shape;
  if (!shape && groupSchema?._def?.innerType) {
    shape = groupSchema._def.innerType.shape || groupSchema._def.innerType._def?.shape;
  }
  if (!shape) return;
  const missingKeys: string[] = [];

  for (const fieldKey of Object.keys(shape)) {
    const key = `${groupId}:${fieldKey}`;
    if (!registeredFieldsMap.has(key)) {
      missingKeys.push(fieldKey);
    }
  }

  if (missingKeys.length > 0) {
    throw new Error(
      `Cannot register config group: field(s) [${missingKeys.join(", ")}] are missing or not registered in configFieldRegistry for groupId "${groupId}".`,
    );
  }
}

const origGroupAdd = configGroupRegistry.add.bind(configGroupRegistry);
configGroupRegistry.add = function (schema: any, meta: any) {
  if (meta && typeof meta === "object" && meta.id) {
    if (schema) {
      validateGroupFields(schema, meta.id);
    }
    registeredGroupsMap.set(meta.id, { schema, meta });
  }
  return origGroupAdd(schema, meta);
};
