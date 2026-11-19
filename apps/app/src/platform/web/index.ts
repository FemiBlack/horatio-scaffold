import type { AppInfo, IStorageService, ISessionService } from "../types";

export const isTauri = false;

// Targets the Axum backend (proxied via Vite's `/api` in dev, or configured via VITE_API_URL)
const API_BASE = import.meta.env.VITE_API_URL || "/api";

// Calls Axum GET /health to fetch domain AppInfo from crates/common
export async function getAppInfo(): Promise<AppInfo> {
  const res = await fetch(`${API_BASE}/health`);
  if (!res.ok) {
    throw new Error(`HTTP ${res.status} ${res.statusText}`);
  }
  const data = await res.json();
  return data.app;
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

// Fetch-based session stub contacting backend auth/session endpoint
export const session: ISessionService = {
  async getToken(): Promise<string | null> {
    const res = await fetch(`${API_BASE}/session`);
    if (!res.ok) {
      throw new Error(`HTTP ${res.status} ${res.statusText}`);
    }
    const data = await res.json();
    return data.token ?? null;
  },
  async setToken(token: string): Promise<void> {
    const res = await fetch(`${API_BASE}/session`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ token }),
    });
    if (!res.ok) {
      throw new Error(`HTTP ${res.status} ${res.statusText}`);
    }
  },
  async clearToken(): Promise<void> {
    const res = await fetch(`${API_BASE}/session`, { method: "DELETE" });
    if (!res.ok) {
      throw new Error(`HTTP ${res.status} ${res.statusText}`);
    }
  },
};
