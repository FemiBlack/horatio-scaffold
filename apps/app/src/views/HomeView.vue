<script setup lang="ts">
import { isTauri, storage, session, getAppInfo } from "@platform";
import { ref, onMounted } from "vue";

const platformTitle = isTauri ? "running in Tauri" : "running in browser";
const platformDescription = isTauri
  ? "Running inside the Tauri v2 native shell"
  : "Running in the web browser (PWA mode)";

const storageStatus = ref<string>("testing...");
const sessionStatus = ref<string>("testing...");
const appInfoResult = ref<string>("loading...");

onMounted(async () => {
  // Test storage
  try {
    await storage.setItem("demo_key", "Operational");
    const val = await storage.getItem("demo_key");
    storageStatus.value = val ?? "null";
  } catch (err: any) {
    storageStatus.value = `Failed: ${err?.message ?? err}`;
  }

  // Test session
  try {
    await session.setToken("scaffold_token_xyz");
    const token = await session.getToken();
    sessionStatus.value = token ?? "null";
  } catch (err: any) {
    sessionStatus.value = `Failed: ${err?.message ?? err}`;
  }

  // Test shared domain AppInfo
  try {
    const info = await getAppInfo();
    appInfoResult.value = `${info.name} v${info.version}`;
  } catch (err: any) {
    appInfoResult.value = `Failed: ${err?.message ?? err}`;
  }
});
</script>

<template>
  <main class="container">
    <div class="card">
      <div class="pill" :class="isTauri ? 'pill-tauri' : 'pill-web'">
        {{ isTauri ? 'Tauri Shell' : 'Web Browser' }}
      </div>

      <h1 class="title">{{ platformTitle }}</h1>
      <p class="description">{{ platformDescription }}</p>

      <section class="info-section">
        <h2 class="section-title">Platform Inspection</h2>
        <div class="row">
          <span class="label">Source:</span>
          <code class="value">import { isTauri } from "@platform"</code>
        </div>
        <div class="row">
          <span class="label">isTauri:</span>
          <code class="value">{{ isTauri }}</code>
        </div>
        <div class="row">
          <span class="label">Storage test:</span>
          <span class="value" :class="{ error: storageStatus.startsWith('Failed') }">{{ storageStatus }}</span>
        </div>
        <div class="row">
          <span class="label">Session test:</span>
          <span class="value" :class="{ error: sessionStatus.startsWith('Failed') }">{{ sessionStatus }}</span>
        </div>
        <div class="row">
          <span class="label">Shared AppInfo:</span>
          <span class="value" :class="{ error: appInfoResult.startsWith('Failed') }">{{ appInfoResult }}</span>
        </div>
      </section>
    </div>
  </main>
</template>

<style scoped>
.container {
  min-height: 100vh;
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 2rem;
  background-color: #f8fafc;
  font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial, sans-serif;
  color: #0f172a;
}

.card {
  background: #ffffff;
  border: 1px solid #e2e8f0;
  border-radius: 12px;
  box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.05), 0 2px 4px -2px rgba(0, 0, 0, 0.05);
  padding: 2.5rem;
  max-width: 540px;
  width: 100%;
}

.pill {
  display: inline-block;
  padding: 0.25rem 0.75rem;
  border-radius: 9999px;
  font-size: 0.75rem;
  font-weight: 600;
  text-transform: uppercase;
  letter-spacing: 0.05em;
  margin-bottom: 1.25rem;
}

.pill-tauri {
  background-color: #e0e7ff;
  color: #4338ca;
}

.pill-web {
  background-color: #dcfce7;
  color: #15803d;
}

.title {
  font-size: 1.75rem;
  font-weight: 700;
  margin: 0 0 0.5rem;
  line-height: 1.2;
}

.description {
  font-size: 1rem;
  color: #64748b;
  margin: 0 0 1.75rem;
}

.info-section {
  border-top: 1px solid #f1f5f9;
  padding-top: 1.25rem;
}

.section-title {
  font-size: 0.875rem;
  font-weight: 600;
  color: #475569;
  text-transform: uppercase;
  letter-spacing: 0.05em;
  margin: 0 0 1rem;
}

.row {
  display: flex;
  justify-content: space-between;
  align-items: baseline;
  gap: 1rem;
  margin-bottom: 0.75rem;
  font-size: 0.9rem;
}

.label {
  color: #64748b;
  font-weight: 500;
}

.value {
  font-weight: 600;
  text-align: right;
}

.value.error {
  color: #dc2626;
  font-weight: 500;
  font-size: 0.85em;
}

code {
  background: #f1f5f9;
  padding: 0.2rem 0.4rem;
  border-radius: 4px;
  font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, monospace;
  font-size: 0.85em;
}
</style>
