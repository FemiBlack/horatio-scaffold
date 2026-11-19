import type { IStorageService, ISessionService } from "../types";

export const isTauri = false;

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

export const session: ISessionService = {
  async getToken(): Promise<string | null> {
    const res = await fetch("/api/session").catch(() => null);
    if (!res || !res.ok) return null;
    const data = await res.json().catch(() => null);
    return data?.token ?? null;
  },
  async setToken(token: string): Promise<void> {
    await fetch("/api/session", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ token }),
    }).catch(() => {});
  },
  async clearToken(): Promise<void> {
    await fetch("/api/session", { method: "DELETE" }).catch(() => {});
  },
};
