import { readFileSync } from 'node:fs'

let crateVersion: string = 'latest'

// Try GitHub releases first
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
    if (tag) {
      // Normalize common prefixes like v0.1.0
      crateVersion = tag.startsWith('v') ? tag : `v${tag}`
    }
  }
} catch {}

// Fallback: read from Cargo.toml
if (crateVersion === 'latest') {
  try {
    const cargoTomlPath = new URL('../../Cargo.toml', import.meta.url)
    const text = readFileSync(cargoTomlPath, 'utf8')
    const m = text.match(/\bversion\s*=\s*"([^"]+)"/)
    if (m && m[1]) crateVersion = `v${m[1]}`
  } catch {}
}

export { crateVersion }
