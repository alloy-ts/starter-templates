import { createRequire } from "node:module";
import path from "node:path";

const require = createRequire(import.meta.url);

let nativeBinding: any;
try {
  nativeBinding = require("../index.js");
} catch {
  try {
    nativeBinding = require("./index.js");
  } catch {
    nativeBinding = require(path.join(process.cwd(), "index.js"));
  }
}

export const {
  add,
  BuildCommand,
  BuilderCommand,
  BuilderDelete,
  BuilderStart,
  BuilderStatus,
  BuilderStop,
  BuildTransfer,
  ContainerClient,
  ContainerCompose,
  ContainerStart,
  ImageTransfer,
  IO,
  Io = nativeBinding.IO || nativeBinding.Io,
  Platform,
  ProcessHandle,
  ProcessIO,
  ProcessIo = nativeBinding.ProcessIO || nativeBinding.ProcessIo,
  ServerStream,
  VirtcontainersRuntime,
  ContainerStatus,
  PacketType,
} = nativeBinding;

export type {
  BuildCommandOptions,
  BuilderStartOptions,
  ComposeUpOptions,
  Container,
  ContainerConfiguration,
  ContainerStartOptions,
  InitProcess,
  Mount,
} from "../index.d.ts";

export { nativeBinding };
export { Container } from "./cli/container-build.ts";
export * as containerCli from "./cli/container.ts";
export * as containerBuildCli from "./cli/container-build.ts";
export * as containerComposeCli from "./cli/container-compose.ts";
