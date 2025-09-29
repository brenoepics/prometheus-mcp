<script setup lang="ts">
import {onMounted, ref} from 'vue'
import InstallButtons from "./InstallButtons.vue";

const version = ref('latest')

onMounted(async () => {
  try {
    const res = await fetch('https://api.github.com/repos/brenoepics/prometheus-mcp/releases/latest', {
      headers: {
        'Accept': 'application/vnd.github+json',
        'User-Agent': 'prometheus-mcp-docs'
      }
    })
    if (res.ok) {
      const data = await res.json()
      const tag = (data?.tag_name || '').toString()
      if (tag) version.value = tag.startsWith('v') ? tag : `v${tag}`
    }
  } catch {
    // ignore — fallback remains "latest"
  }
})

const dockerUrl = 'https://hub.docker.com/r/brenoepics/prometheus-mcp'
const crateUrl = 'https://crates.io/crates/prometheus-mcp'
const latestUrl = 'https://github.com/brenoepics/prometheus-mcp/releases/latest'
</script>

<template>
  <div class="header-links">
    <install-buttons/>
    <a class="chip docker" :href="dockerUrl" target="_blank" rel="noopener noreferrer">Docker</a>
    <a class="chip crate" :href="crateUrl" target="_blank" rel="noopener noreferrer">Crate</a>
    <a class="chip version" :href="latestUrl" target="_blank" rel="noopener noreferrer">{{ version }}</a>
  </div>
</template>

<style scoped>
.header-links {
  display: none;
  gap: 8px;
}

@media (min-width: 1060px) {
  .header-links {
    display: inline-flex;
    align-items: center;
    margin-inline: 8px;
  }
}

.chip {
  text-decoration: none;
  padding: 6px 10px;
  line-height: 1;
  border-radius: 14px;
  font-size: 13px;
  font-weight: 600;
  border: 1px solid transparent;
  transition: background-color 0.15s ease, border-color 0.15s ease, color 0.15s ease;
}

/* Docker: brand blue */
.chip.docker {
  background: #2496ED;
  color: #fff;
  border-color: #1e7fcb;
}

.chip.docker:hover {
  background: #1e7fcb;
}

/* Crate: rusty orange */
.chip.crate {
  background: #C95D2E;
  color: #fff;
  border-color: #a64c25;
}

.chip.crate:hover {
  background: #a64c25;
}

/* Version: on-brand badge */
.chip.version {
  background: var(--vp-c-brand-soft);
  color: var(--vp-c-brand-2);
  border-color: var(--vp-c-brand-2);
}

.chip.version:hover {
  background: var(--vp-c-brand-3);
  color: #fff;
  border-color: var(--vp-c-brand-2);
}
</style>
