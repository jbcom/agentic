---
title: "meshy-content-generator"
description: Standalone declarative image pipelines backed by vendor-fabric
---

# meshy-content-generator

The former `@jbcom/agentic-meshy` workspace package has moved to the
standalone OSS repository
[`jbcom/meshy-content-generator`](https://github.com/jbcom/meshy-content-generator).

The new Python CLI owns declarative expansion and local ImageMagick steps. It
depends on `vendor-fabric`, which exclusively owns Meshy authentication, HTTP,
rate limiting, errors, polling, downloads and resumable job manifests.

```bash
uvx --from meshy-content-generator content-gen run pipeline.json --root . --dry-run
```

The old npm package is retired and must not be used for new work.

