import { expect, test } from "vite-plus/test";
import { Container } from "./container-build.ts";

test("Container.build execution", () => {
  const result = Container.build({
    contextDir: ".",
    tag: ["app-test:v1"],
  });
  expect(result).toBe("Successfully built image: app-test:v1");

  expect(() => {
    Container.build({
      contextDir: "/non/existent/context/path",
    });
  }).toThrow("context dir does not exist");
});

test("Container.builder methods", () => {
  expect(Container.builder.start({ cpus: 2 })).toBe(true);
  expect(Container.builder.status()).toBe("running");
  expect(Container.builder.stop()).toBe(true);
  expect(Container.builder.delete(true)).toBe(true);
});

test("Container.compose methods", () => {
  expect(Container.compose.up({ projectName: "demo-app" })).toBe(
    "Compose project 'demo-app' started"
  );
  expect(Container.compose.down("demo-app")).toBe(
    "Compose project 'demo-app' stopped"
  );
});
