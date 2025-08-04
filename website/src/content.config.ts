import { defineCollection, reference, z } from "astro:content";
import { glob, file } from "astro/loaders";
import { readdir, readFile } from "fs/promises";
import { join } from "path";

const ruletables_raw = defineCollection({
  loader: async () => {
    const dirPath = "../ruletables/";
    const files = await readdir(dirPath);
    const ruletableFiles = files.filter((file: string) =>
      file.endsWith(".txt"),
    );
    const tables = await Promise.all(
      ruletableFiles.map(async (file: string) => {
        const raw = await readFile(join(dirPath, file), "utf-8");
        return {
          id: file.replace(".txt", ""),
          raw,
        };
      }),
    );
    return tables;
  },
  schema: z.object({
    id: z.string(),
    raw: z.string(),
  }),
});

const ruletables = defineCollection({
  loader: glob({ pattern: "*.{md,mdx}", base: "./src/content/ruletables" }),
  schema: z.object({
    title: z.string(),
    description: z.string(),
    source: reference("ruletables_raw"),
    graph: z.string(),
    references: z.array(reference("references")),
  }),
});

const code_raw = defineCollection({
  loader: async () => {
    const dirPaths = [
      "../examples/ciflypy-examples/ciflypy_examples/",
      "../examples/ciflyr-examples/R/",
    ];

    const tables = await Promise.all(
      dirPaths.flatMap(async (dirPath) => {
        const files = await readdir(dirPath);
        const ruletableFiles = files.filter(
          (file: string) => file.endsWith(".py") || file.endsWith(".R"),
        );

        return Promise.all(
          ruletableFiles.map(async (file: string) => {
            const raw = await readFile(join(dirPath, file), "utf-8");
            return {
              id: file,
              raw,
            };
          }),
        );
      }),
    ).then((results) => results.flat());

    return tables;
  },
  schema: z.object({
    id: z.string(),
    raw: z.string(),
  }),
});

const applications = defineCollection({
  loader: glob({ pattern: "*.{md,mdx}", base: "./src/content/applications" }),
  schema: z.object({
    title: z.string(),
    shortTitle: z.string(),
    summary: z.string(),
    description: z.string(),
    code: z.array(reference("code_raw")),
    ruletables: z.array(reference("ruletables_raw")),
    graph: z.string(),
    references: z.array(reference("references")),
    number: z.number(),
  }),
});

const references = defineCollection({
  loader: file("src/content/references/papers.json"),
  schema: z.object({
    authors: z.array(z.string()),
    title: z.string(),
    year: z.string(),
    venue: z.string(),
    url: z.string(),
  }),
});

const docs = defineCollection({
  loader: glob({ pattern: "*.{md,mdx}", base: "./src/content/docs" }),
  schema: z.object({
    title: z.string(),
    shortTitle: z.string(),
    summary: z.string(),
    description: z.string(),
    number: z.number(),
  }),
});

export const collections = {
  ruletables_raw,
  ruletables,
  code_raw,
  applications,
  references,
  docs,
};
