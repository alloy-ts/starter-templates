import {
  BuildCommand,
  BuilderCommand,
  ContainerClient,
  ContainerCompose,
  ContainerStart,
  type BuildCommandOptions,
  type BuilderStartOptions,
  type ComposeUpOptions,
  type ContainerStartOptions,
} from "../lib.ts";

export const Container = {
  build(options: BuildCommandOptions) {
    return BuildCommand.run(options);
  },
  builder: {
    status() {
      return BuilderCommand.status();
    },
    start(options?: BuilderStartOptions) {
      return BuilderCommand.start(options);
    },
    stop() {
      return BuilderCommand.stop();
    },
    delete(force?: boolean) {
      return BuilderCommand.delete(force);
    },
  },
  start(client: ContainerClient, options: ContainerStartOptions) {
    return ContainerStart.run(client, options);
  },
  compose: {
    up(options?: ComposeUpOptions) {
      return ContainerCompose.up(options);
    },
    down(projectName?: string) {
      return ContainerCompose.down(projectName);
    },
  },
};

export { BuildCommand, BuilderCommand };
