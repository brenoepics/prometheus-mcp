// Sidebar configuration grouped by sections
const sidebar = [
  {
    text: 'Guide',
    items: [
      { text: 'Installation', link: '/installation' },
      { text: 'Getting Started', link: '/usage' },
      { text: 'Configuration', link: '/configuration' },
      { text: 'Tools', link: '/tools' },
      { text: 'Docker', link: '/docker' },
      { text: 'Claude Desktop', link: '/claude-desktop' },
      { text: 'Debugging', link: '/debugging' }
    ]
  },
  {
    text: 'Installing on Clients',
    items: [
      { text: 'VS Code', link: '/clients/vscode' },
      { text: 'Cursor', link: '/clients/cursor' },
      { text: 'Zed', link: '/clients/zed' },
      { text: 'Windsurf', link: '/clients/windsurf' },
      { text: 'MCP Inspector', link: '/clients/inspector' },
    ]
  }
]

export default sidebar
