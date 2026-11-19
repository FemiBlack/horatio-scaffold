import type { AppInfo, IStorageService, ISessionService } from "../types";

export const isTauri = false;

// Demonstrates retrieving domain types shared across crates/common
export async function getAppInfo(): Promise<AppInfo> {
  const res = await fetch("/health").catch(() => null);
  if (res?.ok) {
    const data = await res.json().catch(() => null);
    if (data?.app) return data.app;
  }
  return { name: "horatio-scaffold", version: "0.1.0" };
}

export const storage: IStorageService = {
  async getItem(key: string): Promise<string | null> {
    return localStorage.getItem(key);
  },
  async setItem(key: string, value: string): Promise<void> {
    localStorage.setItem(key, value);
  },
  async removeItem(key: string): Promise<void> {
    localStorage.removeItem(key);
  },
};

// Illustrative session service stub:
// In production, this typically communicates with an auth endpoint (e.g. /api/session).
// Here we use localStorage as an in-browser fallback for local demo testing.
const SESSION_KEY = "demo_session_token";

export const session: ISessionService = {
  async getToken(): Promise<string | null> {
    // Illustrative HTTP fetch with fallback:
    const res = await fetch("/api/session").catch(() => null);
    if (res?.ok) {
      const data = await res.json().catch(() => null);
      return data?.token ?? null;
    }
    return localStorage.getItem(SESSION_KEY);
  },
  async setToken(token: string): Promise<void> {
    await fetch("/api/session", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ token }),
    }).catch(() => {});
    localStorage.setItem(SESSION_KEY, token);
  },
  async clearToken(): Promise<void> {
    await fetch("/api/session", { method: "DELETE" }).catch(() => {});
    localStorage.removeItem(SESSION_KEY);
  },
};
