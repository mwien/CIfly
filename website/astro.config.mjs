// @ts-check
import { defineConfig } from "astro/config";
import mdx from "@astrojs/mdx";
import remarkMath from "remark-math";
import rehypeKatex from "rehype-katex";
import tailwindcss from "@tailwindcss/vite";
import CiflyGrammar from "./src/shiki/grammar/cifly.tmLanguage.json";
import customCatppucinLatte from "./src/shiki/theme/catppuccin-latte.json";
import customCatppucinMocha from "./src/shiki/theme/catppuccin-mocha.json";

import sitemap from "@astrojs/sitemap";

// https://astro.build/config
export default defineConfig({
  markdown: {
    remarkPlugins: [remarkMath],
    rehypePlugins: [rehypeKatex],
    shikiConfig: {
      defaultColor: false,
      langs: [{ ...CiflyGrammar }, "r", "python", "bibtex"],
      themes: { light: customCatppucinLatte, dark: customCatppucinMocha },
    },
  },
  site: "https://cifly.pages.dev",
  integrations: [mdx(), sitemap()],
  trailingSlash: "always",
  vite: {
    plugins: [tailwindcss()],
  },
});
