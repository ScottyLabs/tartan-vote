const envBase = import.meta.env.VITE_API_BASE;
let configuredBase = "";
if (envBase) {
  configuredBase = envBase.replace(/\/$/u, "");
}

const apiBaseFromHostname = (hostname: string): string => {
  if (hostname === "tartan-vote.scottylabs.org") {
    return "https://api.tartan-vote.scottylabs.org";
  }

  const preview = hostname.match(/^tartan-vote-frontend-(?<slug>.+)\.scottylabs\.net$/u);
  if (preview && preview.groups && preview.groups.slug) {
    return `https://tartan-vote-tartan-vote-${preview.groups.slug}.scottylabs.net`;
  }

  return "";
};

const { hostname } = globalThis.location;

/** API origin; empty string means same-origin (local dev with Vite proxy). */
export const apiBase = configuredBase || apiBaseFromHostname(hostname);

/** Resolve an API path against the configured base URL. */
export const apiUrl = (path: string): string => {
  if (/^https?:\/\//u.test(path)) {
    return path;
  }
  let normalized = path;
  if (!normalized.startsWith("/")) {
    normalized = `/${normalized}`;
  }
  return `${apiBase}${normalized}`;
};
