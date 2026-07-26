/// <reference types="vite/client" />

const envBase: unknown = import.meta.env.VITE_API_BASE;
const configuredBase = typeof envBase === "string" ? envBase.replace(/\/$/u, "") : "";

export const apiBase = configuredBase;

export const apiUrl = (path: string): string => {
  if (/^https?:\/\//u.test(path)) {
    return path;
  }

  const normalized = path.startsWith("/") ? path : `/${path}`;
  const apiPath =
    normalized === "/api" || normalized.startsWith("/api/") ? normalized : `/api${normalized}`;
  return `${apiBase}${apiPath}`;
};
