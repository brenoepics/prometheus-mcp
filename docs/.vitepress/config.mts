import { defineConfigWithTheme, type DefaultTheme } from 'vitepress'
import type { ThemeConfig } from 'vitepress-carbon'
import baseConfig from 'vitepress-carbon/config'
import { version } from '../package.json'
import nav from './nav.mts'
import sidebar from './sidebar.mts'

// https://vitepress.dev/reference/site-config
export default defineConfigWithTheme<ThemeConfig & { themeConfig: DefaultTheme.Config }>({
  extends: baseConfig,
  lang: 'en-US',
  title: 'prometheus-mcp',
  description: 'Prometheus MCP server + CLI tools with retries, caching, and an optional metrics exporter.',
  srcDir: 'src',
  base: '/prometheus-mcp/',
  lastUpdated: true,
  cleanUrls: true,
  metaChunk: true,
  ignoreDeadLinks: 'localhostLinks',

  sitemap: {
    hostname: 'https://brenoepics.github.io/prometheus-mcp/'
  },

  head: [
    ['link', { rel: 'icon', href: '/bg.svg' }],
    ['meta', { name: 'theme-color', content: '#E6522C' }],
    ['meta', { property: 'og:url', content: 'https://github.com/brenoepics/prometheus-mcp' }],
    ['meta', { property: 'og:type', content: 'Repository' }],
    ['meta', { property: 'og:title', content: 'prometheus-mcp' }],
    ['meta', { property: 'og:description', content: 'Prometheus MCP server + CLI tools with retries, caching, and an optional metrics exporter.' }]
  ],

  themeConfig: {
    nav: [
      ...nav,
      { text: 'v' + version, link: 'https://github.com/brenoepics/prometheus-mcp/releases', activeMatch: '^/$' }
    ],
    sidebar,

    logo: {
      alt: 'prometheus-mcp Logo',
      src: '/bg.svg'
    },

    search: { provider: 'local' },

    outline: [2, 3],

    socialLinks: [
      { icon: 'github', link: 'https://github.com/brenoepics/prometheus-mcp' }
    ],

    editLink: {
      pattern: 'https://github.com/brenoepics/prometheus-mcp/edit/main/docs/src/:path',
      text: 'Edit this page on GitHub'
    },

    footer: {
      message: 'Apache-2.0 Licensed'
    }
  }
})
