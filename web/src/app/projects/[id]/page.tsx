"use client";

import { useEffect, useState } from "react";
import { useParams, useRouter } from "next/navigation";
import { api } from "@/lib/api";
import type { Project, Environment, Secret, SecretValue } from "@/lib/types";
import { maskValue, formatDate } from "@/lib/utils";
import {
  ArrowLeft,
  Plus,
  Eye,
  EyeOff,
  Copy,
  Trash2,
  Search,
  Check,
  Key,
  Folder,
} from "lucide-react";

export default function ProjectPage() {
  const params = useParams();
  const router = useRouter();
  const projectId = params.id as string;

  const [project, setProject] = useState<Project | null>(null);
  const [environments, setEnvironments] = useState<Environment[]>([]);
  const [activeEnv, setActiveEnv] = useState<Environment | null>(null);
  const [secrets, setSecrets] = useState<Secret[]>([]);
  const [revealedIds, setRevealedIds] = useState<Set<string>>(new Set());
  const [secretValues, setSecretValues] = useState<Map<string, string>>(new Map());
  const [copiedId, setCopiedId] = useState<string | null>(null);
  const [search, setSearch] = useState("");
  const [showAdd, setShowAdd] = useState(false);
  const [newKey, setNewKey] = useState("");
  const [newValue, setNewValue] = useState("");
  const [newDesc, setNewDesc] = useState("");
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    loadData();
  }, [projectId]);

  useEffect(() => {
    if (activeEnv) loadSecrets(activeEnv.id);
  }, [activeEnv]);

  async function loadData() {
    try {
      const projects = await api.getProjects();
      const proj = projects.find((p) => p.id === projectId);
      if (!proj) {
        router.push("/");
        return;
      }
      setProject(proj);

      const envs = await api.getEnvironments(projectId);
      setEnvironments(envs);
      if (envs.length > 0) setActiveEnv(envs[0]);
    } catch (err) {
      console.error("Failed to load project:", err);
    } finally {
      setLoading(false);
    }
  }

  async function loadSecrets(envId: string) {
    try {
      const data = await api.getSecrets(envId);
      setSecrets(data);
      setRevealedIds(new Set());
      setSecretValues(new Map());
    } catch (err) {
      console.error("Failed to load secrets:", err);
    }
  }

  async function toggleReveal(secret: Secret) {
    if (revealedIds.has(secret.id)) {
      setRevealedIds((prev) => {
        const next = new Set(prev);
        next.delete(secret.id);
        return next;
      });
    } else {
      if (!secretValues.has(secret.id)) {
        try {
          const data = await api.getSecretValue(secret.id);
          setSecretValues((prev) => new Map(prev).set(secret.id, data.value));
        } catch (err) {
          console.error("Failed to get secret value:", err);
          return;
        }
      }
      setRevealedIds((prev) => new Set(prev).add(secret.id));
    }
  }

  async function copyValue(secret: Secret) {
    let value = secretValues.get(secret.id);
    if (!value) {
      try {
        const data = await api.getSecretValue(secret.id);
        value = data.value;
        setSecretValues((prev) => new Map(prev).set(secret.id, value!));
      } catch {
        return;
      }
    }
    await navigator.clipboard.writeText(value);
    setCopiedId(secret.id);
    setTimeout(() => setCopiedId(null), 2000);
  }

  async function deleteSecret(id: string) {
    if (!confirm("Delete this secret?")) return;
    try {
      await api.deleteSecret(id);
      if (activeEnv) loadSecrets(activeEnv.id);
    } catch (err) {
      console.error("Failed to delete secret:", err);
    }
  }

  async function handleAdd() {
    if (!newKey.trim() || !newValue || !activeEnv) return;
    try {
      await api.createSecret(activeEnv.id, newKey.trim(), newValue, newDesc.trim() || undefined);
      setNewKey("");
      setNewValue("");
      setNewDesc("");
      setShowAdd(false);
      loadSecrets(activeEnv.id);
    } catch (err) {
      console.error("Failed to create secret:", err);
    }
  }

  const filtered = secrets.filter(
    (s) =>
      s.key.toLowerCase().includes(search.toLowerCase()) ||
      s.description?.toLowerCase().includes(search.toLowerCase())
  );

  if (loading) {
    return (
      <div className="flex min-h-screen items-center justify-center bg-surface-secondary">
        <div className="flex flex-col items-center gap-3">
          <div className="h-8 w-8 animate-spin rounded-full border-2 border-primary border-t-transparent" />
          <span className="text-sm text-text-tertiary">Loading project...</span>
        </div>
      </div>
    );
  }

  return (
    <div className="min-h-screen bg-surface-secondary">
      {/* Header */}
      <header className="sticky top-0 z-40 border-b border-border bg-white/80 backdrop-blur-xl">
        <div className="mx-auto flex h-16 max-w-6xl items-center gap-4 px-6">
          <button
            onClick={() => router.push("/")}
            className="rounded-lg p-2 text-text-tertiary hover:text-text-primary hover:bg-surface-tertiary transition-colors duration-150 cursor-pointer"
          >
            <ArrowLeft className="h-5 w-5" />
          </button>
          <div className="flex items-center gap-3">
            <div className="flex h-8 w-8 items-center justify-center rounded-lg bg-primary/10">
              <Folder className="h-4 w-4 text-primary" />
            </div>
            <div>
              <h1 className="text-base font-semibold text-text-primary">{project?.name}</h1>
              {project?.description && (
                <p className="text-xs text-text-tertiary">{project.description}</p>
              )}
            </div>
          </div>
        </div>
      </header>

      <main className="mx-auto max-w-6xl px-6 py-8">
        {/* Environment tabs */}
        <div className="animate-fade-in">
          <div className="inline-flex items-center gap-1 rounded-xl bg-surface-tertiary p-1 border border-border-light">
            {environments.map((env) => (
              <button
                key={env.id}
                onClick={() => setActiveEnv(env)}
                className={`
                  rounded-lg px-4 py-2 text-sm font-medium transition-all duration-150 cursor-pointer
                  ${activeEnv?.id === env.id
                    ? "bg-white text-text-primary shadow-sm"
                    : "text-text-secondary hover:text-text-primary"
                  }
                `}
              >
                <span className="capitalize">{env.name}</span>
                <span className={`ml-2 text-xs ${activeEnv?.id === env.id ? 'text-primary' : 'text-text-tertiary'}`}>
                  {env.secret_count}
                </span>
              </button>
            ))}
          </div>
        </div>

        {/* Toolbar */}
        <div className="mt-6 flex items-center justify-between gap-4 animate-fade-in stagger-1">
          <div className="relative flex-1 max-w-sm">
            <Search className="absolute left-3.5 top-1/2 h-4 w-4 -translate-y-1/2 text-text-tertiary" />
            <input
              type="text"
              placeholder="Filter secrets..."
              value={search}
              onChange={(e) => setSearch(e.target.value)}
              className="input pl-10"
            />
          </div>
          <button
            onClick={() => setShowAdd(true)}
            className="btn-primary"
          >
            <Plus className="h-4 w-4" />
            Add Secret
          </button>
        </div>

        {/* Secrets table */}
        <div className="mt-4 card overflow-hidden animate-fade-in stagger-2">
          <table className="w-full">
            <thead>
              <tr className="border-b border-border-light">
                <th className="px-5 py-3.5 text-left text-label text-text-tertiary uppercase font-medium w-56">Key</th>
                <th className="px-5 py-3.5 text-left text-label text-text-tertiary uppercase font-medium">Value</th>
                <th className="px-5 py-3.5 text-left text-label text-text-tertiary uppercase font-medium w-36">Updated</th>
                <th className="px-5 py-3.5 w-28"></th>
              </tr>
            </thead>
            <tbody className="divide-y divide-border-light">
              {filtered.map((secret, index) => {
                const isRevealed = revealedIds.has(secret.id);
                const value = secretValues.get(secret.id);

                return (
                  <tr
                    key={secret.id}
                    className="group hover:bg-surface-secondary/50 transition-colors"
                    style={{ animationDelay: `${index * 30}ms` }}
                  >
                    <td className="px-5 py-4">
                      <div className="flex items-center gap-2.5">
                        <div className="flex h-7 w-7 items-center justify-center rounded-md bg-surface-tertiary">
                          <Key className="h-3.5 w-3.5 text-text-tertiary" />
                        </div>
                        <div>
                          <div className="font-mono text-sm font-medium text-text-primary">
                            {secret.key}
                          </div>
                          {secret.description && (
                            <div className="text-xs text-text-tertiary mt-0.5">
                              {secret.description}
                            </div>
                          )}
                        </div>
                      </div>
                    </td>
                    <td className="px-5 py-4">
                      <code className="text-sm font-mono text-text-secondary bg-surface-tertiary px-2.5 py-1 rounded-md">
                        {isRevealed && value ? value : maskValue("secret")}
                      </code>
                    </td>
                    <td className="px-5 py-4 text-xs text-text-tertiary">
                      {formatDate(secret.updated_at)}
                    </td>
                    <td className="px-5 py-4">
                      <div className="flex items-center gap-1 opacity-0 group-hover:opacity-100 transition-opacity duration-150">
                        <button
                          onClick={() => toggleReveal(secret)}
                          className="rounded-lg p-1.5 text-text-tertiary hover:text-text-primary hover:bg-surface-tertiary transition-colors cursor-pointer"
                          title={isRevealed ? "Hide" : "Reveal"}
                        >
                          {isRevealed ? (
                            <EyeOff className="h-4 w-4" />
                          ) : (
                            <Eye className="h-4 w-4" />
                          )}
                        </button>
                        <button
                          onClick={() => copyValue(secret)}
                          className="rounded-lg p-1.5 text-text-tertiary hover:text-text-primary hover:bg-surface-tertiary transition-colors cursor-pointer"
                          title="Copy"
                        >
                          {copiedId === secret.id ? (
                            <Check className="h-4 w-4 text-emerald-500" />
                          ) : (
                            <Copy className="h-4 w-4" />
                          )}
                        </button>
                        <button
                          onClick={() => deleteSecret(secret.id)}
                          className="rounded-lg p-1.5 text-text-tertiary hover:text-red-500 hover:bg-red-50 transition-colors cursor-pointer"
                          title="Delete"
                        >
                          <Trash2 className="h-4 w-4" />
                        </button>
                      </div>
                    </td>
                  </tr>
                );
              })}
              {filtered.length === 0 && (
                <tr>
                  <td colSpan={4} className="px-5 py-16 text-center">
                    <div className="flex flex-col items-center gap-3">
                      <div className="flex h-12 w-12 items-center justify-center rounded-xl bg-surface-tertiary">
                        <Key className="h-6 w-6 text-text-tertiary" />
                      </div>
                      <div>
                        <p className="text-sm font-medium text-text-primary">
                          {search ? "No secrets match your filter" : "No secrets yet"}
                        </p>
                        <p className="text-xs text-text-tertiary mt-1">
                          {search ? "Try a different search term" : "Add your first secret to get started"}
                        </p>
                      </div>
                      {!search && (
                        <button
                          onClick={() => setShowAdd(true)}
                          className="btn-primary mt-2"
                        >
                          <Plus className="h-4 w-4" />
                          Add Secret
                        </button>
                      )}
                    </div>
                  </td>
                </tr>
              )}
            </tbody>
          </table>
        </div>
      </main>

      {/* Add secret modal */}
      {showAdd && (
        <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/40 backdrop-blur-sm animate-fade-in">
          <div className="w-full max-w-md rounded-xl bg-white p-6 shadow-elevated animate-scale-in">
            <h2 className="text-heading-2 text-text-primary">Add Secret</h2>
            <p className="mt-1 text-body-small text-text-secondary">
              Add a new secret to the <span className="font-medium text-text-primary">{activeEnv?.name}</span> environment.
            </p>
            <div className="mt-5 space-y-3">
              <div>
                <label className="text-label text-text-secondary mb-1.5 block">Key</label>
                <input
                  type="text"
                  placeholder="e.g. DATABASE_URL"
                  value={newKey}
                  onChange={(e) => setNewKey(e.target.value.toUpperCase().replace(/[^A-Z0-9_]/g, "_"))}
                  className="input font-mono"
                  autoFocus
                />
              </div>
              <div>
                <label className="text-label text-text-secondary mb-1.5 block">Value</label>
                <input
                  type="password"
                  placeholder="Enter the secret value"
                  value={newValue}
                  onChange={(e) => setNewValue(e.target.value)}
                  className="input"
                />
              </div>
              <div>
                <label className="text-label text-text-secondary mb-1.5 block">Description <span className="text-text-tertiary">(optional)</span></label>
                <input
                  type="text"
                  placeholder="What is this secret for?"
                  value={newDesc}
                  onChange={(e) => setNewDesc(e.target.value)}
                  className="input"
                />
              </div>
            </div>
            <div className="mt-6 flex justify-end gap-3">
              <button
                onClick={() => setShowAdd(false)}
                className="btn-secondary"
              >
                Cancel
              </button>
              <button
                onClick={handleAdd}
                disabled={!newKey.trim() || !newValue}
                className="btn-primary disabled:opacity-50 disabled:cursor-not-allowed"
              >
                Add Secret
              </button>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}
