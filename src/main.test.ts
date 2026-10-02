import { expect, test } from "vite-plus/test";
import {
  add,
  BuildCommand,
  BuilderCommand,
  BuilderDelete,
  BuilderStart,
  BuilderStatus,
  BuilderStop,
  BuildTransfer,
  Container,
  ContainerClient,
  ContainerStart,
  ContainerStatus,
  ImageTransfer,
  Io,
  PacketType,
  Platform,
  ProcessIo,
  ServerStream,
  VirtcontainersRuntime,
  type Container as ContainerType,
} from "./lib.ts";

test("add function", () => {
  expect(add(2, 3)).toBe(5);
});

test("BuildTransfer metadata parsing", () => {
  const bt = new BuildTransfer({
    stage: "builder",
    method: "dockerfile",
    "include-patterns": "src/*, package.json",
    followpaths: "node_modules",
    mode: "0755",
    size: "1024",
    offset: "512",
    length: "2048",
  });

  expect(bt.stage()).toBe("builder");
  expect(bt.method()).toBe("dockerfile");
  expect(bt.includePatterns()).toEqual(["src/*", "package.json"]);
  expect(bt.followPaths()).toEqual(["node_modules"]);
  expect(bt.mode()).toBe("0755");
  expect(bt.size()).toBe(1024);
  expect(bt.offset()).toBe(512);
  expect(bt.len()).toBe(2048);
});

test("ImageTransfer metadata parsing", () => {
  const it = new ImageTransfer({
    stage: "pull",
    method: "registry",
    ref: "ubuntu:latest",
    platform: "linux/arm64/v8",
    mode: "read",
    size: "4096",
    length: "1024",
    offset: "0",
  });

  expect(it.stage()).toBe("pull");
  expect(it.method()).toBe("registry");
  expect(it.refName()).toBe("ubuntu:latest");
  const platform = it.platform();
  expect(platform).not.toBeNull();
  expect(platform?.os).toBe("linux");
  expect(platform?.arch).toBe("arm64");
  expect(platform?.variant).toBe("v8");
  expect(it.mode()).toBe("read");
  expect(it.size()).toBe(4096);
  expect(it.len()).toBe(1024);
  expect(it.offset()).toBe(0);
});

test("ServerStream packet type handling", () => {
  const io = new Io("stdio-stream");
  const stream = new ServerStream(PacketType.IO, null, null, io);

  expect(stream.getPacketType()).toBe(PacketType.IO);
  expect(stream.getIo()?.stdio).toBe("stdio-stream");
  expect(stream.getImageTransfer()).toBeNull();
  expect(stream.getBuildTransfer()).toBeNull();
});

test("ContainerStart logic and errors", () => {
  const client = new ContainerClient();

  const containerRunning: ContainerType = {
    id: "cont-running-1",
    status: ContainerStatus.Running,
    configuration: {
      mounts: [],
      initProcess: { terminal: true },
    },
  };
  client.registerContainer(containerRunning);

  expect(() => {
    ContainerStart.run(client, {
      attach: true,
      interactive: false,
      containerId: "cont-running-1",
    });
  }).toThrow("attach is currently unsupported on already running containers");

  const runningId = ContainerStart.run(client, {
    attach: false,
    interactive: false,
    containerId: "cont-running-1",
  });
  expect(runningId).toBe("cont-running-1");

  const containerWithMount: ContainerType = {
    id: "cont-mount-1",
    status: ContainerStatus.Stopped,
    configuration: {
      mounts: [
        {
          source: "/non/existent/path/for/virtiofs/mount",
          target: "/mnt/data",
          isVirtiofs: true,
        },
      ],
      initProcess: { terminal: false },
    },
  };
  client.registerContainer(containerWithMount);

  expect(() => {
    ContainerStart.run(client, {
      attach: false,
      interactive: false,
      containerId: "cont-mount-1",
    });
  }).toThrow("mount source path '/non/existent/path/for/virtiofs/mount' does not exist");
});

test("BuildCommand execution", () => {
  const result = BuildCommand.run({
    contextDir: ".",
    tag: ["my-image:v1.0.0"],
  });
  expect(result).toBe("Successfully built image: my-image:v1.0.0");

  expect(() => {
    BuildCommand.run({
      contextDir: "/non/existent/context/dir",
    });
  }).toThrow("context dir does not exist");
});

test("Container global object wrapper", () => {
  const buildResult = Container.build({
    contextDir: ".",
    tag: ["wrapper-image:v1.0.0"],
  });
  expect(buildResult).toBe("Successfully built image: wrapper-image:v1.0.0");

  expect(Container.builder.status()).toBe("running");
  expect(Container.builder.start({ cpus: 2 })).toBe(true);
  expect(Container.builder.stop()).toBe(true);
  expect(Container.builder.delete(false)).toBe(true);

  expect(Container.compose.up({ projectName: "my-app" })).toBe("Compose project 'my-app' started");
  expect(Container.compose.down("my-app")).toBe("Compose project 'my-app' stopped");
});

test("Builder commands", () => {
  expect(BuilderStart.start({ cpus: 2, memory: "2048MB" })).toBe(true);
  expect(BuilderStatus.status()).toBe("running");
  expect(BuilderStop.stop()).toBe(true);
  expect(BuilderDelete.delete(true)).toBe(true);

  expect(BuilderCommand.status()).toBe("running");
  expect(BuilderCommand.start({ cpus: 4 })).toBe(true);
  expect(BuilderCommand.stop()).toBe(true);
  expect(BuilderCommand.delete(false)).toBe(true);
});

test("ProcessIo creation and close", () => {
  const pio = ProcessIo.create(true, false, true);
  expect(pio.tty).toBe(true);
  expect(pio.interactive).toBe(false);
  expect(pio.detach).toBe(true);
  expect(pio.closed).toBe(false);
  pio.close();
  expect(pio.closed).toBe(true);
});

test("VirtcontainersRuntime bindings", () => {
  const runtime = new VirtcontainersRuntime();
  expect(runtime.active).toBe(true);
  expect(runtime.version()).toBe("runtime-rs/0.1.0");
  expect(runtime.createSandbox("test-sb")).toBe("sandbox-test-sb");
  expect(runtime.startSandbox("test-sb")).toBe(true);
  expect(runtime.stopSandbox("test-sb")).toBe(true);
});
