const API_URL = process.env.NEXT_PUBLIC_API_URL || "http://localhost:3001";

async function request<T>(
  path: string,
  options?: RequestInit
): Promise<T> {
  const res = await fetch(`${API_URL}${path}`, {
    headers: {
      "Content-Type": "application/json",
      ...options?.headers,
    },
    ...options,
  });

  if (!res.ok) {
    const error = await res.json().catch(() => ({ error: "Unknown error" }));
    throw new Error(error.error || `HTTP ${res.status}`);
  }

  return res.json();
}

// Vault
export const api = {
  health: () => request<{ status: string; version: string }>("/api/health"),

  vaultStatus: () => request<{ initialized: boolean; locked: boolean }>("/api/vault/status"),

  unlock: (password: string) =>
    request<{ message: string }>("/api/vault/unlock", {
      method: "POST",
      body: JSON.stringify({ password }),
    }),

  lock: () =>
    request<{ message: string }>("/api/vault/lock", {
      method: "POST",
    }),

  // Projects
  getProjects: () => request<Project[]>("/api/projects"),

  createProject: (name: string, description?: string) =>
    request<Project>("/api/projects", {
      method: "POST",
      body: JSON.stringify({ name, description }),
    }),

  deleteProject: (id: string) =>
    request<{ message: string }>(`/api/projects/${id}`, {
      method: "DELETE",
    }),

  // Environments
  getEnvironments: (projectId: string) =>
    request<Environment[]>(`/api/projects/${projectId}/environments`),

  createEnvironment: (projectId: string, name: string) =>
    request<Environment>(`/api/projects/${projectId}/environments`, {
      method: "POST",
      body: JSON.stringify({ name }),
    }),

  // Secrets
  getSecrets: (environmentId: string) =>
    request<Secret[]>(`/api/secrets?environment_id=${environmentId}`),

  createSecret: (environmentId: string, key: string, value: string, description?: string) =>
    request<{ message: string }>("/api/secrets", {
      method: "POST",
      body: JSON.stringify({ environment_id: environmentId, key, value, description }),
    }),

  updateSecret: (id: string, value?: string, description?: string) =>
    request<{ message: string }>(`/api/secrets/${id}`, {
      method: "PUT",
      body: JSON.stringify({ value, description }),
    }),

  deleteSecret: (id: string) =>
    request<{ message: string }>(`/api/secrets/${id}`, {
      method: "DELETE",
    }),

  getSecretValue: (id: string) =>
    request<SecretValue>(`/api/secrets/${id}/value`),

  // Export
  exportSecrets: (environmentId: string) =>
    request<{ secrets: Array<{ key: string; value: string }> }>(
      `/api/export?environment_id=${environmentId}`
    ),
};

import type { Project, Environment, Secret, SecretValue } from "./types";
