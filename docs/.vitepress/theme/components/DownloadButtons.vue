<template>
  <div class="download-cta" role="region" aria-label="Quick downloads">
    <h3 class="title">Add prometheus-mcp to your client</h3>
    <div class="btn-grid">
      <a
          v-for="client in clients"
          :key="client.name"
          :class="['btn-card', client.class]"
          :href="client.href"
          target="_blank"
          rel="noopener noreferrer"
          :aria-label="client.aria ?? `Open ${client.name}`"
      >
        <span class="label">{{ client.name }}</span>
      </a>
    </div>
    <p class="hint">
      One-click install or docs links. See <a href="/installation">Installation</a> for details and Linux notes.
    </p>
  </div>
</template>

<script setup lang="ts">
import {reactive} from 'vue'

interface Client {
  name: string
  href: string
  aria?: string
  class?: string
}

const clients = reactive<Client[]>([
  { name: 'VS Code', href: 'clients/vscode', aria: 'Open VS Code install guide', class: 'vscode' },
  { name: 'Cursor', href: 'clients/cursor', aria: 'Open Cursor install guide', class: 'cursor' },
  { name: 'Zed', href: 'clients/zed', aria: 'Open Zed install guide', class: 'zed' },
  { name: 'Windsurf', href: 'clients/windsurf', aria: 'Open Windsurf install guide', class: 'windsurf' },
  { name: 'MCP Inspector', href: 'clients/inspector', aria: 'Open MCP Inspector guide', class: 'inspector' },
  { name: 'Claude Desktop', href: 'claude-desktop', aria: 'Open Claude Desktop guide', class: 'guide' },
  { name: 'Docker', href: 'docker', aria: 'Open Docker guide', class: 'docker' },
])
</script>

<style scoped>
.download-cta {
  display: grid;
  gap: 1rem;
  padding: 1.25rem;
  border-radius: 12px;
  background: linear-gradient(180deg, rgba(16, 16, 16, 0.06), rgba(16, 16, 16, 0.02));
  border: 1px solid var(--vp-c-divider);
  width: 90%;
  max-width: clamp(520px, 42vw, 640px);
  z-index: 1;
}

.title {
  margin: 0;
  font-size: 1rem;
  font-weight: 600;
  color: var(--vp-c-text-1);
}

.btn-grid {
  display: grid;
  grid-template-columns: repeat(2, minmax(0, 1fr));
  gap: 0.75rem;
}

.btn-card {
  display: flex;
  align-items: center;
  gap: 0.6rem;
  padding: 0.9rem 1rem;
  border-radius: 10px;
  color: var(--vp-c-text-1);
  text-decoration: none;
  background: var(--vp-c-bg-soft);
  border: 1px solid var(--vp-c-divider);
  transition: transform 0.15s ease, box-shadow 0.15s ease, background-color 0.15s ease, border-color 0.15s ease;
  box-shadow: 0 1px 2px rgba(0, 0, 0, 0.04);
  height: 52px; /* enforce equal height */
}

/* Ensure hover/focus applies to the entire card (default/fallback) */
.btn-card:hover,
.btn-card:focus-visible {
  transform: translateY(-1px);
  background: var(--vp-c-bg);
  border-color: var(--vp-c-brand-1);
  box-shadow: 0 8px 24px rgba(0, 0, 0, 0.08);
  outline: none;
}

/* Equal text handling */
.label {
  font-weight: 600;
  letter-spacing: .2px;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

/* Brand hover/focus styles */
.btn-card.vscode:hover,
.btn-card.vscode:focus-visible {
  background: #007ACC; /* VS Code blue */
  border-color: #007ACC;
  color: #fff;
}

.btn-card.cursor:hover,
.btn-card.cursor:focus-visible {
  background: #111; /* Cursor dark */
  border-color: #111;
  color: #fff;
}

.btn-card.docker:hover,
.btn-card.docker:focus-visible {
  background: #2496ED; /* Docker blue */
  border-color: #1e7fcb;
  color: #fff;
}

/* Guide/default brand */
.btn-card.guide:hover,
.btn-card.guide:focus-visible {
  background: var(--vp-c-brand-2);
  border-color: var(--vp-c-brand-2);
  color: #fff;
}

/* Zed (red accent) */
.btn-card.zed:hover,
.btn-card.zed:focus-visible {
  background: #E53935; /* Zed red-ish */
  border-color: #C62828;
  color: #fff;
}

/* Windsurf (violet accent) */
.btn-card.windsurf:hover,
.btn-card.windsurf:focus-visible {
  background: #8B5CF6; /* violet */
  border-color: #7C3AED;
  color: #fff;
}

/* MCP Inspector (cyan accent) */
.btn-card.inspector:hover,
.btn-card.inspector:focus-visible {
  background: #06B6D4; /* cyan */
  border-color: #0891B2;
  color: #fff;
}

@media (max-width: 1000px) {
  .download-cta { display: none; }
}
</style>
