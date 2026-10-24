import type { Config } from "tailwindcss";

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
          DEFAULT: "#0B0D14",
          deep: "#07080D",
          surface: "#151928",
          border: "#283048",
          muted: "#A8B5C8",
          text: "#F8FAFC",
          hover: "#1E2438",
          active: "#261E45",
        },
        stone: {
          50: "#F8FAFC",
          100: "#F1F5F9",
          200: "#E2E8F0",
          300: "#CBD5E1",
          400: "#94A3B8",
          500: "#64748B",
          canvas: "#090A0F",
          surface: "#121622",
          card: "#161B2A",
          border: "#262D42",
          muted: "#A8B5C8",
          ink: "#F8FAFC",
        },
        burnt: {
          DEFAULT: "#7C3AED",
          hover: "#6D28D9",
          light: "#231942",
        },
        success: {
          bg: "rgba(16, 185, 129, 0.16)",
          ink: "#6EE7B7",
        },
        warning: {
          bg: "rgba(245, 158, 11, 0.16)",
          ink: "#FCD34D",
        },
        danger: {
          bg: "rgba(239, 68, 68, 0.16)",
          ink: "#FCA5A5",
        },
        dm: {
          obsidian: "#090A0F",
          surface: "#121622",
          elevated: "#171C2B",
          border: "#262D42",
          violet: "#7C3AED",
          cyan: "#06B6D4",
          amber: "#F59E0B",
        },
      },
      fontFamily: {
        sans: [
          "Plus Jakarta Sans",
          "Inter",
          "-apple-system",
          "BlinkMacSystemFont",
          "Segoe UI",
          "Roboto",
          "sans-serif",
        ],
        display: [
          "Space Grotesk",
          "Plus Jakarta Sans",
          "Inter",
          "sans-serif",
        ],
        mono: ["JetBrains Mono", "ui-monospace", "SFMono-Regular", "monospace"],
      },
      borderRadius: {
        DEFAULT: "8px",
      },
    },
  },
  plugins: [],
};

export default config;
