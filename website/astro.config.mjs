// @ts-check
import { defineConfig } from "astro/config";
import mdx from "@astrojs/mdx";
import remarkMath from "remark-math";
import rehypeKatex from "rehype-katex";
import tailwindcss from "@tailwindcss/vite";
import CiflyGrammar from "./src/shiki/grammar/cifly.tmLanguage.json";
import CustomCatppucinLatte from "./src/shiki/theme/catppuccin-latte.json";

// https://astro.build/config
export default defineConfig({
  markdown: {
    remarkPlugins: [remarkMath],
    rehypePlugins: [rehypeKatex],
    shikiConfig: {
      langs: [{ ...CiflyGrammar }],
      theme: { ...CustomCatppucinLatte },
    },
  },
  integrations: [mdx()], // TODO: sitemap?
  trailingSlash: "always",
  vite: {
    plugins: [tailwindcss()],
  },
});
