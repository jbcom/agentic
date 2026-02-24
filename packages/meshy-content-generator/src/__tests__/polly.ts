import path from "node:path";
import FetchAdapter from "@pollyjs/adapter-fetch";
import { Polly } from "@pollyjs/core";
import FSPersister from "@pollyjs/persister-fs";

Polly.register(FetchAdapter);
Polly.register(FSPersister);

export function createPolly(recordingName: string) {
  const recordingsDir = path.resolve(process.cwd(), "test/__recordings__");
  const mode = process.env.POLLY_MODE ?? "replay";

  const polly = new Polly(recordingName, {
    adapters: ["fetch"],
    persister: "fs",
    mode,
    logLevel: "silent",
    recordIfMissing: mode === "record",
    matchRequestsBy: {
      headers: {
        exclude: ["authorization"],
      },
    },
    persisterOptions: {
      fs: {
        recordingsDir,
      },
    },
  });

  polly.server.any().on("beforePersist", (_, recording) => {
    const entries = Array.isArray(recording.entries)
      ? recording.entries
      : ((recording as { log?: { entries?: [] } }).log?.entries ?? []);
    for (const entry of entries) {
      if (entry.request.headers?.authorization) {
        entry.request.headers.authorization = "[REDACTED]";
      }
    }
  });

  return polly;
}
