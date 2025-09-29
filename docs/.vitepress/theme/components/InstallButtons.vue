<script setup lang="ts">
import {onBeforeUnmount, onMounted, ref} from 'vue'

const open = ref(false)
const container = ref<HTMLElement | null>(null)

const toggle = (e: MouseEvent) => {
  e.stopPropagation()
  open.value = !open.value
}

const onClickOutside = (e: MouseEvent) => {
  if (!container.value) return
  if (!container.value.contains(e.target as Node)) {
    open.value = false
  }
}

onMounted(() => document.addEventListener('click', onClickOutside))
onBeforeUnmount(() => document.removeEventListener('click', onClickOutside))

const cursorUrl = 'https://cursor.com/en/install-mcp?name=prometheus-mcp&config=%7B%22command%22%3A%22docker%22%2C%22args%22%3A%5B%22run%22%2C%22--rm%22%2C%22-i%22%2C%22brenoepics%2Fprometheus-mcp%3Alatest%22%2C%22--mcp%22%2C%22--prometheus-url%22%2C%22http%3A%2F%2Fhost.docker.internal%3A9090%22%5D%7D'
const vscodeUrl = 'https://insiders.vscode.dev/redirect/mcp/install?name=prometheus%20mcp%20server&config=%7B%22command%22%3A%22docker%22%2C%22args%22%3A%5B%22run%22%2C%22--rm%22%2C%22-i%22%2C%22brenoepics%2Fprometheus-mcp%3Alatest%22%2C%22--mcp%22%2C%22--prometheus-url%22%2C%22http%3A%2F%2Fhost.docker.internal%3A9090%22%5D%7D'
</script>

<template>
  <div ref="container" class="install-buttons">
    <button class="add-ide" @click="toggle">
      Add to IDE
      <svg class="caret" viewBox="0 0 16 16" width="14" height="14" aria-hidden="true">
        <path fill="currentColor"
              d="M4.427 6.427a.75.75 0 0 1 1.146-.976L8 8.243l2.427-2.792a.75.75 0 0 1 1.146.976l-3 3.455a.75.75 0 0 1-1.146 0z"/>
      </svg>
    </button>
    <div v-show="open" class="dropdown" @click.stop>
      <a class="item" :href="vscodeUrl" target="_blank" rel="noopener noreferrer">
        <span class="dot vscode" aria-hidden="true"></span>
        <span class="text">Install for VS Code</span>
      </a>
      <a class="item" :href="cursorUrl" target="_blank" rel="noopener noreferrer">
        <span class="dot cursor" aria-hidden="true"></span>
        <span class="text">Install for Cursor</span>
      </a>
    </div>
  </div>
</template>

<style scoped>
.install-buttons {
  position: relative;
  display: none;
}

@media (min-width: 1060px) {
  .install-buttons {
    display: inline-block;
  }
}

.add-ide {
  text-decoration: none;
  padding: 2px 6px;
  color: #fff;
  font-size: 14px;
  text-align: center;
  border-radius: 5px;
  background-color: #00863f;
  border: 1px solid #1A9B57;
  transition: background-color 0.2s ease, box-shadow 0.2s ease;
  line-height: 1.4;
  white-space: nowrap;
  font-family: inherit;
  font-weight: 600;
  display: inline-flex;
  align-items: center;
  gap: 6px;
}

.add-ide:hover {
  background-color: #009044;
}

.add-ide:active {
  background-color: #29903B;
}

.add-ide .caret {
  opacity: 0.85;
}

.dropdown {
  position: absolute;
  right: 0;
  top: calc(100% + 6px);
  min-width: 240px;
  padding: 8px;
  border-radius: 8px;
  background: var(--vp-c-bg);
  border: 1px solid var(--vp-c-divider);
  box-shadow: 0 10px 20px rgba(0, 0, 0, 0.08);
  z-index: 1000;
}

.item {
  display: flex;
  align-items: center;
  gap: 10px;
  text-decoration: none;
  font-weight: 600;
  padding: 10px 12px;
  border-radius: 6px;
  color: var(--vp-c-text-1);
  border: 1px solid transparent;
}

.item:hover {
  background: var(--vp-c-bg-alt);
  border-color: var(--vp-c-divider);
}

.dot {
  width: 10px;
  height: 10px;
  border-radius: 50%;
  display: inline-block;
}

.dot.vscode {
  background: #007ACC;
}

.dot.cursor {
  background: #111;
}

.text {
  line-height: 1;
}
</style>
