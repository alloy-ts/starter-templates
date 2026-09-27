import {
  buildGroupSchemaFromFields,
  getFieldsForGroupId,
  isFieldRegistered,
  registeredGroupsMap,
  type MetadataConfigGroup,
} from "./registries.ts";

export type Scope = "local" | "user" | "system";

/**
 * Stores configuration values per group ID and scope.
 */
const scopeStores = new Map<string, Record<Scope, Record<string, any>>>();

/**
 * WeakMap association for Config instances to hold metadata & resolveMap.
 */
const configInstanceMetadata = new WeakMap<
  Config,
  {
    groupId: string;
    meta: MetadataConfigGroup;
    resolveMap?: MetadataConfigGroup["resolveMap"];
  }
>();

/**
 * Resolver registry per groupId.
 */
export const resolver: Record<string, { resolveMap?: MetadataConfigGroup["resolveMap"] }> = {};

function getOrCreateScopeStore(groupId: string): Record<Scope, Record<string, any>> {
  let store = scopeStores.get(groupId);
  if (!store) {
    store = {
      local: {},
      user: {},
      system: {},
    };
    scopeStores.set(groupId, store);
  }
  return store;
}

/**
 * Seed initial values for a group in a given scope.
 */
export function seedScopeStore(groupId: string, scope: Scope, values: Record<string, any>): void {
  const store = getOrCreateScopeStore(groupId);
  Object.assign(store[scope], values);
}

/**
 * Helper to strip prefixes and non-alphanumeric delimiters for flexible matching.
 */
function normalizeIdentifier(str: string): string {
  return str
    .replace(/^urn:/i, "")
    .replace(/^config\./i, "")
    .replace(/[:._-]/g, "")
    .toLowerCase();
}

/**
 * Resolves a group entry from registered groups by string identifier (id, urn, or normalized name).
 */
export function resolveGroup(groupIdentifier: string): { schema: any; meta: MetadataConfigGroup } {
  const query = groupIdentifier.trim();
  const normalizedQuery = normalizeIdentifier(query);

  for (const entry of registeredGroupsMap.values()) {
    const { meta } = entry;
    if (
      meta.id === query ||
      meta.urn === query ||
      normalizeIdentifier(meta.urn) === normalizedQuery ||
      normalizeIdentifier(meta.id) === normalizedQuery
    ) {
      const groupSchema = entry.schema || buildGroupSchemaFromFields(meta.id);
      return { schema: groupSchema, meta };
    }
  }

  // Check if fields exist for this groupId directly
  const fields = getFieldsForGroupId(query);
  if (fields.size > 0) {
    const meta: MetadataConfigGroup = {
      id: query,
      urn: query,
    };
    const groupSchema = buildGroupSchemaFromFields(query);
    return { schema: groupSchema, meta };
  }

  throw new Error(`Config group "${groupIdentifier}" not found in registry.`);
}

/**
 * Parses a full key / URN string into its group entry and optional field key.
 */
export function parseConfigKey(fullKey: string): {
  group: { schema: any; meta: MetadataConfigGroup };
  fieldKey?: string;
} {
  try {
    const group = resolveGroup(fullKey);
    return { group };
  } catch {
    // Continue
  }

  const splitIndices: number[] = [];
  for (let i = fullKey.length - 1; i >= 0; i--) {
    if (fullKey[i] === ":" || fullKey[i] === ".") {
      splitIndices.push(i);
    }
  }

  for (const idx of splitIndices) {
    const groupCandidate = fullKey.slice(0, idx);
    const fieldCandidate = fullKey.slice(idx + 1);

    if (!groupCandidate || !fieldCandidate) continue;

    try {
      const group = resolveGroup(groupCandidate);
      return { group, fieldKey: fieldCandidate };
    } catch {
      // Try next
    }
  }

  throw new Error(`Config key or URN "${fullKey}" could not be resolved.`);
}

/**
 * Creates a strongly-typed `defineConfig` function for a config group ID string or schema object.
 */
export function createDefineConfig(schemaOrGroupId: any): (config: any) => any {
  let schema: any;

  if (typeof schemaOrGroupId === "string") {
    schema = resolveGroup(schemaOrGroupId).schema;
  } else if (schemaOrGroupId && typeof schemaOrGroupId.parse === "function") {
    schema = schemaOrGroupId;
  } else {
    throw new Error("Invalid arguments for createDefineConfig");
  }

  return function defineConfig(configValue: any): any {
    return schema.parse(configValue);
  };
}

/**
 * Group Resolver providing scope-aware `.get()` and `.set()` methods for a specific config group.
 */
export class ConfigGroupResolver {
  public readonly groupMeta: MetadataConfigGroup;
  public readonly groupSchema: any;

  constructor(groupIdentifier: string) {
    const { meta, schema } = resolveGroup(groupIdentifier);
    this.groupMeta = meta;
    this.groupSchema = schema;

    if (!resolver[meta.id]) {
      resolver[meta.id] = { resolveMap: meta.resolveMap };
    }
  }

  private isFieldValid(fieldKey: string): boolean {
    if (isFieldRegistered(this.groupMeta.id, fieldKey)) {
      return true;
    }
    let shape = this.groupSchema?.shape || this.groupSchema?._def?.shape;
    if (!shape && this.groupSchema?._def?.innerType) {
      shape = this.groupSchema._def.innerType.shape || this.groupSchema._def.innerType._def?.shape;
    }
    return !!(shape && fieldKey in shape);
  }

  public get(key?: string, scope?: Scope): any {
    const store = getOrCreateScopeStore(this.groupMeta.id);

    if (key) {
      if (!this.isFieldValid(key)) {
        throw new Error(`Config field "${key}" not found in config group "${this.groupMeta.id}".`);
      }

      if (scope) {
        return store[scope][key];
      }

      if (store.local[key] !== undefined) return store.local[key];
      if (store.user[key] !== undefined) return store.user[key];
      if (store.system[key] !== undefined) return store.system[key];

      return undefined;
    }

    let shape = this.groupSchema?.shape || this.groupSchema?._def?.shape;
    if (!shape && this.groupSchema?._def?.innerType) {
      shape = this.groupSchema._def.innerType.shape || this.groupSchema._def.innerType._def?.shape;
    }

    const keys = shape ? Object.keys(shape) : [];
    const result: Record<string, any> = {};

    for (const k of keys) {
      result[k] = this.get(k, scope);
    }

    return result;
  }

  public set(key: string, value: any, scope: Scope = "local"): void {
    if (!this.isFieldValid(key)) {
      throw new Error(`Config field "${key}" not found in config group "${this.groupMeta.id}".`);
    }

    const store = getOrCreateScopeStore(this.groupMeta.id);
    store[scope][key] = value;
  }
}

/**
 * Global Config class and instance constructor.
 */
export class Config {
  static resolver = resolver;
  static createDefine = createDefineConfig;

  static get(key: string, scope?: Scope): any {
    const { group, fieldKey } = parseConfigKey(key);
    const groupResolver = new ConfigGroupResolver(group.meta.id);
    return groupResolver.get(fieldKey, scope);
  }

  static set(key: string, value: any, scope: Scope = "local"): void {
    const { group, fieldKey } = parseConfigKey(key);
    if (!fieldKey) {
      throw new Error(`Cannot set value on config group "${key}" without specifying a field key.`);
    }
    const groupResolver = new ConfigGroupResolver(group.meta.id);
    groupResolver.set(fieldKey, value, scope);
  }

  private groupResolver: ConfigGroupResolver;

  constructor(groupId: string, resolveMap?: MetadataConfigGroup["resolveMap"]) {
    this.groupResolver = new ConfigGroupResolver(groupId);
    const meta = this.groupResolver.groupMeta;
    if (resolveMap) {
      meta.resolveMap = { ...meta.resolveMap, ...resolveMap };
      const resolverEntry = resolver[meta.id];
      if (resolverEntry) {
        resolverEntry.resolveMap = meta.resolveMap;
      }
    }
    configInstanceMetadata.set(this, {
      groupId: meta.id,
      meta,
      resolveMap: meta.resolveMap,
    });
  }

  get(key?: string, scope?: Scope): any {
    return this.groupResolver.get(key, scope);
  }

  set(key: string, value: any, scope: Scope = "local"): void {
    this.groupResolver.set(key, value, scope);
  }
}

export function get(key: string, scope?: Scope): any {
  return Config.get(key, scope);
}

export function set(key: string, value: any, scope: Scope = "local"): void {
  Config.set(key, value, scope);
}
