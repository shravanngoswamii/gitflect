// @ts-check
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import sitemap from "@astrojs/sitemap";
import { defineConfig } from "astro/config";

const __dirname = dirname(fileURLToPath(import.meta.url));

// https://astro.build/config
export default defineConfig({
	site: "https://shravangoswami.com",
	base: process.env.BASE_PATH ?? "/",
	integrations: [sitemap()],
	vite: {
		server: {
			fs: {
				allow: [resolve(__dirname, "..")],
			},
		},
	},
	markdown: {
		shikiConfig: {
			themes: {
				light: "github-light",
				dark: "github-dark",
			},
			defaultColor: false,
			wrap: false,
		},
	},
});
