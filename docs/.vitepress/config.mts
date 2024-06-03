import { defineConfig } from 'vitepress'

// https://vitepress.dev/reference/site-config
export default defineConfig({
  title: "Bussin WebX Manual & Developer Docs",
  description: "User manual for WebX user & Documentation for WebX developers",
  themeConfig: {
    // https://vitepress.dev/reference/default-theme-config
    nav: [
      { text: 'Home', link: '/' },
      { text: 'User manual', link: '/user-manual' }
    ],

    sidebar: [
      {
        text: 'Examples',
        items: [
          { text: 'Markdown Examples', link: '/markdown-examples' },
          { text: 'Runtime API Examples', link: '/api-examples' }
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
