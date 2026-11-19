import { invoke } from "@tauri-apps/api/core";
import type { AppInfo, IStorageService, ISessionService } from "../types";

export const isTauri = true;

// Calls native Tauri command `get_app_info` returning AppInfo from crates/common
export async function getAppInfo(): Promise<AppInfo> {
  return invoke<AppInfo>("get_app_info");
}

export const storage: IStorageService = {
  async getItem(key: string): Promise<string | null> {
    return invoke<string | null>("get_storage_item", { key }).catch(() => null);
  },
  async setItem(key: string, value: string): Promise<void> {
    await invoke("set_storage_item", { key, value }).catch(() => {});
  },
  async removeItem(key: string): Promise<void> {
    await invoke("remove_storage_item", { key }).catch(() => {});
  },
};

export const session: ISessionService = {
  async getToken(): Promise<string | null> {
    return invoke<string | null>("get_session_token").catch(() => null);
  },
  async setToken(token: string): Promise<void> {
    await invoke("set_session_token", { token }).catch(() => {});
  },
  async clearToken(): Promise<void> {
    await invoke("clear_session_token").catch(() => {});
  },
};
