const assert = require("node:assert/strict");
const test = require("node:test");

const {
  add,
  VirtcontainersRuntime,
  ContainerClient,
  ContainerStatus,
  ContainerStart,
  BuildCommand,
  BuilderCommand,
  BuilderStart,
  BuilderStatus,
} = require("./index.js");

test("adds two numbers", () => {
  assert.equal(add(2, 3), 5);
});

test("virtcontainers runtime", () => {
  const runtime = new VirtcontainersRuntime();
  assert.equal(runtime.version(), "runtime-rs/0.1.0");
});

test("container client and start", () => {
  const client = new ContainerClient();
  client.registerContainer({
    id: "c-100",
    status: ContainerStatus.Stopped,
    configuration: {
      mounts: [],
      initProcess: { terminal: false }
    }
  });

  const res = ContainerStart.run(client, {
    attach: false,
    interactive: false,
    containerId: "c-100"
  });
  assert.equal(res, "c-100");
});

test("build and builder commands", () => {
  const buildRes = BuildCommand.run({
    contextDir: ".",
    tag: ["app:latest"]
  });
  assert.equal(buildRes, "Successfully built image: app:latest");

  assert.equal(BuilderStart.start({ cpus: 2 }), true);
  assert.equal(BuilderStatus.status(), "running");
  assert.equal(BuilderCommand.status(), "running");
});
