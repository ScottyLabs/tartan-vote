const PUBLIC_HOST = "tartan.vote";

const envBase = import.meta.env.VITE_API_BASE;
const configuredBase = envBase ? envBase.replace(/\/$/u, "") : "";

const apiBaseFromHostname = (hostname: string): string => {
  if (hostname === PUBLIC_HOST || hostname === `www.${PUBLIC_HOST}`) {
    return `https://api.${PUBLIC_HOST}`;
  }

  const preview = hostname.match(/^tartan-vote-frontend-(?<slug>.+)\.scottylabs\.net$/u);
  if (preview?.groups?.slug) {
    return `https://tartan-vote-tartan-vote-${preview.groups.slug}.scottylabs.net`;
  }

  return "";
};

const hostname = globalThis.location?.hostname ?? "";

export const apiBase = apiBaseFromHostname(hostname) || configuredBase;

export const apiUrl = (path: string): string => {
  if (/^https?:\/\//u.test(path)) {
    return path;
  }

  const normalized = path.startsWith("/") ? path : `/${path}`;
  return `${apiBase}${normalized}`;
};
