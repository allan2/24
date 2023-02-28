import react from "@vitejs/plugin-react";
import { defineConfig } from "vite";
import eslint from "vite-plugin-eslint";
import wasm from "vite-plugin-wasm";

export default defineConfig({
	build: {
		target: "esnext",
	},
	base: "/24/",
	plugins: [react(), wasm(), eslint()],
});
