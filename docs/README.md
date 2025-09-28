# prometheus-mcp docs

Documentation for the prometheus-mcp project — a Prometheus MCP server + CLI tools with retries, caching, and an optional metrics exporter.

## Quick start

::: tip
Use your preferred package manager. We recommend pnpm.
:::

::: code-group
```bash [pnpm]
pnpm install
```
```bash [npm]
npm install
```
```bash [yarn]
yarn install
```
```bash [bun]
bun install
```
:::

Start the local docs site:

::: code-group
```bash [pnpm]
pnpm run docs:dev
```
```bash [npm]
npm run docs:dev
```
```bash [yarn]
yarn docs:dev
```
```bash [bun]
bun run docs:dev
```
:::

Open http://localhost:5173.

## Build & preview

```bash
pnpm run docs:build
pnpm run docs:preview
```

## Useful links

- GitHub: https://github.com/brenoepics/prometheus-mcp
- VitePress: https://vitepress.dev
- Theme: https://brenoepics.github.io/vitepress-carbon/
