import type { Config } from "tailwindcss";

const color = (token: string) => `rgb(var(--${token}) / <alpha-value>)`;

const config: Config = {
  content: [
    "./src/pages/**/*.{js,ts,jsx,tsx,mdx}",
    "./src/components/**/*.{js,ts,jsx,tsx,mdx}",
    "./src/app/**/*.{js,ts,jsx,tsx,mdx}",
  ],
  theme: {
    extend: {
      colors: {
        charcoal: {
          DEFAULT: color("charcoal"),
          deep: color("canvas"),
          surface: color("surface"),
          border: color("border"),
          muted: color("muted"),
          text: color("ink"),
          hover: color("hover"),
          active: color("accent-surface"),
        },
        stone: {
          50: "#FAF8F7", 100: "#EDE8E7", 200: "#DDD8D7",
          300: "#C5BFC0", 400: "#A9A3A4", 500: "#8C8689",
          canvas: color("canvas"),
          surface: color("surface"),
          card: color("elevated"),
          border: color("border"),
          muted: color("muted"),
          ink: color("ink"),
        },
        crimson: {
          50: "#FFF1F3", 100: "#FFE5EA", 200: "#FFC9D2",
          300: "#FFA2B2", 400: "#F97790", 500: "#E84463",
          600: color("accent"), 700: color("accent-hover"),
          800: "#851C31", 900: "#501422", 950: color("accent-surface"),
        },
        success: { bg: "rgba(16, 185, 129, 0.16)", ink: "#6EE7B7" },
        warning: { bg: "rgba(245, 158, 11, 0.16)", ink: "#FCD34D" },
        danger: { bg: "rgba(239, 68, 68, 0.16)", ink: "#FCA5A5" },
      },
      fontFamily: {
        sans: ["Plus Jakarta Sans", "Inter", "-apple-system", "BlinkMacSystemFont", "Segoe UI", "Roboto", "sans-serif"],
        display: ["Space Grotesk", "Plus Jakarta Sans", "Inter", "sans-serif"],
        mono: ["JetBrains Mono", "ui-monospace", "SFMono-Regular", "monospace"],
      },
      borderRadius: { DEFAULT: "8px" },
    },
  },
  plugins: [],
};

export default config;
