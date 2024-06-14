import { defineConfig } from 'vitepress'

// https://vitepress.dev/reference/site-config
export default defineConfig({
  title: "OpenUnit",
  description: "OpenUnit is a collection of unit converter functions maintained by open-source enthusiasts, designed to facilitate easy conversion between various units of measurement.",
  themeConfig: {
    // https://vitepress.dev/reference/default-theme-config
    nav: [
      { text: 'Home', link: '/' },
      { text: 'Library', link: '/lib' },
      { text: 'CLI', link: '/cli' },
      { text: 'Releases', link: 'https://github.com/kemalcelikk/openunit/releases' }
    ],

    sidebar: [
      {
        text: 'Library',
        link: '/lib',
        items: [
          { text: 'Angle', link: '/lib/angle' },
          { text: 'Area', link: '/lib/area' },
          { text: 'Electricity', link: '/lib/electricity' },
          { text: 'Frequency', link: '/lib/frequency' },
          { text: 'Length', link: '/lib/length' },
          { text: 'Light', link: '/lib/light' },
          { text: 'Mass', link: '/lib/mass' },
          { text: 'Pressure', link: '/lib/pressure' },
          { text: 'Speed', link: '/lib/speed' },
          { text: 'Storage', link: '/lib/storage' },
          { text: 'Tempature', link: '/lib/tempature' },
          { text: 'Time', link: '/lib/time' },
        ]
      },
      {
        text: 'Command-Line Interface',
        link: '/cli'
      }
    ],

    socialLinks: [
      { icon: 'github', link: 'https://github.com/vuejs/vitepress' }
    ]
  }
})
