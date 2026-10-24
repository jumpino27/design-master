/** @type {import('next').NextConfig} */
const isProdBuild =
  process.env.NODE_ENV === "production" ||
  process.env.NEXT_OUTPUT_EXPORT === "true";

const nextConfig = {
  distDir: process.env.DESIGN_MASTER_NEXT_DIST_DIR || ".next",
  ...(isProdBuild ? { output: "export" } : {}),
  images: {
    unoptimized: true,
  },
  eslint: {
    ignoreDuringBuilds: true,
  },
  ...(!isProdBuild
    ? {
        async rewrites() {
          const backendUrl =
            process.env.DESIGN_GEN_BACKEND_URL || "http://127.0.0.1:4318";
          return [
            {
              source: "/api/:path*",
              destination: `${backendUrl}/api/:path*`,
            },
            {
              source: "/media/:path*",
              destination: `${backendUrl}/media/:path*`,
            },
          ];
        },
      }
    : {}),
};

export default nextConfig;
