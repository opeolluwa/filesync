/** @type {import('next').NextConfig} */

const nextConfig = {
  reactStrictMode: true,
  // Note: This feature is required to use NextJS Image in SSG mode.
  // See https://nextjs.org/docs/messages/export-image-api for different workarounds.
  images: {
    unoptimized: true,
  },
  async redirects() {
    return [
      {
        source: "/",
        destination: "/connection",
        permanent: true,
      },
    ];
  },
};

module.exports = nextConfig;
