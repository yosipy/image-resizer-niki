import { defineConfig } from "vitepress"
import { fileURLToPath } from "url"

// https://vitepress.dev/reference/site-config
export default defineConfig({
  title: "image-resizer-niki",
  description: "Resize image on web browser.",
  vite: {
    resolve: {
      alias: {
        "@lib": fileURLToPath(new URL("../../lib", import.meta.url)),
      },
    },
  },
  themeConfig: {
    // https://vitepress.dev/reference/default-theme-config
    nav: [
      { text: "Home", link: "/" },
      { text: "Examples", link: "/examples/markdown-examples" },
    ],

    sidebar: [
      {
        text: "Examples",
        items: [
          { text: "Markdown Examples", link: "/examples/markdown-examples" },
          { text: "Runtime API Examples", link: "/examples/api-examples" },
        ],
      },
    ],

    socialLinks: [
      { icon: "github", link: "https://github.com/yosipy/image-resizer-niki" },
    ],
  },
})
