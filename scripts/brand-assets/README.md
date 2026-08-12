# Brand asset generation

The declarative `pipeline.json` and `catalogue.json` run through the standalone
`meshy-content-generator`; this monorepo contains no Meshy client.

```bash
cp .env.example .env
bash scripts/brand-assets/generate.sh --dry-run
bash scripts/brand-assets/generate.sh agentic-hero
```

`--dry-run` validates all three requests without reading the API key, making a
network call, or writing output. The live route delegates credentials, retries,
polling, errors, downloads and job sidecars to `vendor-fabric` through the
standalone package. Generated images land under `generated/`.
