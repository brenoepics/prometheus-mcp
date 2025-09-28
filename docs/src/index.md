---
# https://brenoepics.github.io/vitepress-carbon/guide/home-component.html
layout: home

hero:
  name: "prometheus-mcp"
  text: "Prometheus MCP server + CLI tools"
  tagline: Query Prometheus from CLI or any MCP client, with retries, caching, and an optional metrics exporter.
  icon: 🛠️
  image:
    src: /bg.svg
    alt: Banner
  actions:
    - theme: brand
      text: Get Started
      link: /usage
    - theme: alt
      text: Configuration
      link: /configuration
    - theme: alt
      text: Tools
      link: /tools

features:
  - title: Prometheus tools
    details: Instant and range queries, list metrics, series selectors, label values, and metric metadata.
  - title: MCP server
    details: JSON-RPC over stdio for use with Model Context Protocol clients like Claude Desktop.
  - title: Robust I/O
    details: Built-in timeouts, retries, backoff, optional query rate-limiting, and simple caching.
  - title: Metrics exporter
    details: Expose internal /metrics for self-observability with a single flag.
  - title: Docker-friendly
    details: Small runtime image and examples for Linux, macOS, and Windows hosts.
  - title: Secure by default
    details: Supports Basic Auth via env or flags; keep secrets out of configs.
---
