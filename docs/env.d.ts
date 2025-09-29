/// <reference types="vite/client" />
/// <reference types="vitepress/client" />

declare module '*.vue' {
  import type { DefineComponent } from 'vue'
  const component: DefineComponent<{}, {}, any>
  export default component
}

declare module 'vitepress-carbon/config' {
    import { UserConfig } from 'vitepress'
    const config: () => Promise<UserConfig>
    export default config
}