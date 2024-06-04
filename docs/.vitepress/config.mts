import { defineConfig } from 'vitepress'

// https://vitepress.dev/reference/site-config
export default defineConfig({
  title: "Bussin WebX Docs",
  lang: "en-US",
  base: "/docs",
  lastUpdated: true,
  description: "User manual for WebX user & Documentation for WebX developers",
  themeConfig: {
    // https://vitepress.dev/reference/default-theme-config
    nav: [
      { text: 'Home', link: '/' },
      { text: 'User manual', link: '/user-manual' },
      { text: 'Dev Docs', link: '/dev-start' }
    ],
    lastUpdated: {
      text: 'Last updated at',
      formatOptions: {
        dateStyle: 'full',
        timeStyle: 'medium'
      }
    },
    sidebar: [
      {
        text: 'For users',
        items: [
          { text: 'First time', link: '/user-manual' },
          { text: 'Advanced options', link: '/user-advanced' }
        ]
      },
      {
        text: 'For developers',
        items: [
          { text: 'Getting started', link: '/dev-start' },
          { text: 'HTML++', link: '/htmlpp' },
          { text: 'CSS 3.25', link: '/css' },
          { text: 'Luau', link: '/luau' },
          { text: 'Site publishing & Domain registering', link: '/dev-publish' },
          { text: 'API Reference', link: '/webx-api' },
        ]
      }
    ],

    socialLinks: [
      { icon: 'github', link: 'https://github.com/face-hh/webx' },
      { icon: 'youtube', link: 'https://youtube.com/@facedevstuff' },
      { icon: 'discord', link: 'discord.gg/W98yWga6YK' },
      { icon: 'twitter', link: 'twitter.com/facedevstuff' }
    ]
  }
})
