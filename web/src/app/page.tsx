"use client";

import { useEffect, useState } from "react";
import { useRouter } from "next/navigation";
import { api } from "@/lib/api";
import type { Project } from "@/lib/types";
import { Plus, Folder, Key, Settings, ArrowRight, LogOut } from "lucide-react";

export default function Dashboard() {
  const router = useRouter();
  const [projects, setProjects] = useState<Project[]>([]);
  const [loading, setLoading] = useState(true);
  const [showCreate, setShowCreate] = useState(false);
  const [newName, setNewName] = useState("");
  const [newDesc, setNewDesc] = useState("");

  useEffect(() => {
    checkVault();
  }, []);

  async function checkVault() {
    try {
      const status = await api.vaultStatus();
      if (!status.initialized || status.locked) {
        router.push("/vault/unlock");
        return;
      }
      loadProjects();
    } catch {
      router.push("/vault/unlock");
    }
  }

  async function loadProjects() {
    try {
      const data = await api.getProjects();
      setProjects(data);
    } catch (err) {
      console.error("Failed to load projects:", err);
    } finally {
      setLoading(false);
    }
  }

  async function handleCreate() {
    if (!newName.trim()) return;
    try {
      await api.createProject(newName.trim(), newDesc.trim() || undefined);
      setNewName("");
      setNewDesc("");
      setShowCreate(false);
      loadProjects();
    } catch (err) {
      console.error("Failed to create project:", err);
    }
  }

  async function handleDelete(id: string) {
    if (!confirm("Delete this project and all its secrets?")) return;
    try {
      await api.deleteProject(id);
      loadProjects();
    } catch (err) {
      console.error("Failed to delete project:", err);
    }
  }

  async function handleLogout() {
    try {
      await api.lock();
      router.push("/vault/unlock");
    } catch (err) {
      console.error("Failed to lock vault:", err);
      router.push("/vault/unlock");
    }
  }

  if (loading) {
    return (
      <div className="flex min-h-screen items-center justify-center bg-surface-secondary">
        <div className="flex flex-col items-center gap-3">
          <div className="h-8 w-8 animate-spin rounded-full border-2 border-primary border-t-transparent" />
          <span className="text-sm text-text-tertiary">Loading...</span>
        </div>
      </div>
    );
  }

  return (
    <div className="min-h-screen bg-surface-secondary">
      {/* Header */}
      <header className="sticky top-0 z-40 border-b border-border bg-white/80 backdrop-blur-xl">
        <div className="mx-auto flex h-16 max-w-6xl items-center justify-between px-6">
          <div className="flex items-center gap-3">
            <div className="flex h-8 w-8 items-center justify-center rounded-lg bg-primary">
              <Key className="h-4 w-4 text-white" />
            </div>
            <span className="text-base font-semibold text-text-primary tracking-tight">GhostKey</span>
          </div>
          <div className="flex items-center gap-2">
            <button
              onClick={() => router.push("/settings")}
              className="rounded-lg p-2 text-text-tertiary hover:text-text-primary hover:bg-surface-tertiary transition-colors duration-150 cursor-pointer"
              title="Settings"
            >
              <Settings className="h-5 w-5" />
            </button>
            <button
              onClick={handleLogout}
              className="rounded-lg p-2 text-text-tertiary hover:text-red-500 hover:bg-red-50 transition-colors duration-150 cursor-pointer"
              title="Lock & Logout"
            >
              <LogOut className="h-5 w-5" />
            </button>
            <button
              onClick={() => setShowCreate(true)}
              className="btn-primary"
            >
              <Plus className="h-4 w-4" />
              New Project
            </button>
          </div>
        </div>
      </header>

      {/* Main */}
      <main className="mx-auto max-w-6xl px-6 py-10">
        <div className="animate-fade-in">
          <h1 className="text-display text-text-primary">Projects</h1>
          <p className="mt-2 text-body text-text-secondary max-w-lg">
            Manage your secrets by project and environment. Keep your API keys, database credentials, and config variables secure.
          </p>
        </div>

        {/* Stats */}
        {projects.length > 0 && (
          <div className="mt-8 grid grid-cols-3 gap-4 animate-fade-in stagger-1">
            <div className="card p-5">
              <div className="text-stat text-primary">{projects.length}</div>
              <div className="mt-1 text-label text-text-tertiary uppercase">Projects</div>
            </div>
            <div className="card p-5">
              <div className="text-stat text-primary">
                {projects.reduce((sum, p) => sum + p.environment_count, 0)}
              </div>
              <div className="mt-1 text-label text-text-tertiary uppercase">Environments</div>
            </div>
            <div className="card p-5">
              <div className="text-stat text-primary">
                {projects.reduce((sum, p) => sum + p.secret_count, 0)}
              </div>
              <div className="mt-1 text-label text-text-tertiary uppercase">Secrets</div>
            </div>
          </div>
        )}

        {/* Project grid */}
        <div className="mt-8 grid gap-4 sm:grid-cols-2 lg:grid-cols-3">
          {projects.map((project, index) => (
            <div
              key={project.id}
              onClick={() => router.push(`/projects/${project.id}`)}
              className="card group cursor-pointer p-6 animate-slide-up"
              style={{ animationDelay: `${index * 60}ms`, animationFillMode: 'backwards' }}
            >
              <div className="flex items-start justify-between">
                <div className="flex h-10 w-10 items-center justify-center rounded-lg bg-primary/10">
                  <Folder className="h-5 w-5 text-primary" />
                </div>
                <button
                  onClick={(e) => {
                    e.stopPropagation();
                    handleDelete(project.id);
                  }}
                  className="opacity-0 group-hover:opacity-100 rounded-lg p-1.5 text-text-tertiary hover:text-red-500 hover:bg-red-50 transition-all duration-150 cursor-pointer"
                >
                  <svg className="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={2}>
                    <path strokeLinecap="round" strokeLinejoin="round" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
                  </svg>
                </button>
              </div>
              <h3 className="mt-4 text-heading-3 text-text-primary">{project.name}</h3>
              {project.description && (
                <p className="mt-1.5 text-body-small text-text-secondary line-clamp-2">{project.description}</p>
              )}
              <div className="mt-4 flex items-center gap-4 text-label text-text-tertiary">
                <span className="flex items-center gap-1.5">
                  <svg className="h-3.5 w-3.5" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={2}>
                    <path strokeLinecap="round" strokeLinejoin="round" d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z" />
                  </svg>
                  {project.environment_count} env{project.environment_count !== 1 ? "s" : ""}
                </span>
                <span className="flex items-center gap-1.5">
                  <svg className="h-3.5 w-3.5" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={2}>
                    <path strokeLinecap="round" strokeLinejoin="round" d="M15 7a2 2 0 012 2m4 0a6 6 0 01-7.743 5.743L11 17H9v2H7v2H4a1 1 0 01-1-1v-2.586a1 1 0 01.293-.707l5.964-5.964A6 6 0 1121 9z" />
                  </svg>
                  {project.secret_count} secret{project.secret_count !== 1 ? "s" : ""}
                </span>
              </div>
              <div className="mt-4 flex items-center gap-1 text-sm font-medium text-primary opacity-0 group-hover:opacity-100 transition-opacity duration-200">
                View project
                <ArrowRight className="h-3.5 w-3.5 transition-transform group-hover:translate-x-0.5" />
              </div>
            </div>
          ))}

          {projects.length === 0 && (
            <div className="col-span-full">
              <div className="card flex flex-col items-center justify-center py-16 px-8 text-center border-dashed">
                <div className="flex h-14 w-14 items-center justify-center rounded-2xl bg-surface-tertiary">
                  <Folder className="h-7 w-7 text-text-tertiary" />
                </div>
                <h3 className="mt-4 text-heading-3 text-text-primary">No projects yet</h3>
                <p className="mt-1.5 text-body-small text-text-secondary max-w-sm">
                  Create your first project to start managing secrets. Each project can have multiple environments like development, staging, and production.
                </p>
                <button
                  onClick={() => setShowCreate(true)}
                  className="btn-primary mt-6"
                >
                  <Plus className="h-4 w-4" />
                  Create your first project
                </button>
              </div>
            </div>
          )}
        </div>
      </main>

      {/* Create modal */}
      {showCreate && (
        <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/40 backdrop-blur-sm animate-fade-in">
          <div className="w-full max-w-md rounded-xl bg-white p-6 shadow-elevated animate-scale-in">
            <h2 className="text-heading-2 text-text-primary">New Project</h2>
            <p className="mt-1 text-body-small text-text-secondary">
              Create a new project to organize your secrets.
            </p>
            <div className="mt-5 space-y-3">
              <div>
                <label className="text-label text-text-secondary mb-1.5 block">Project name</label>
                <input
                  type="text"
                  placeholder="e.g. my-web-app"
                  value={newName}
                  onChange={(e) => setNewName(e.target.value)}
                  className="input"
                  autoFocus
                />
              </div>
              <div>
                <label className="text-label text-text-secondary mb-1.5 block">Description <span className="text-text-tertiary">(optional)</span></label>
                <input
                  type="text"
                  placeholder="What is this project for?"
                  value={newDesc}
                  onChange={(e) => setNewDesc(e.target.value)}
                  className="input"
                />
              </div>
            </div>
            <div className="mt-6 flex justify-end gap-3">
              <button
                onClick={() => setShowCreate(false)}
                className="btn-secondary"
              >
                Cancel
              </button>
              <button
                onClick={handleCreate}
                disabled={!newName.trim()}
                className="btn-primary disabled:opacity-50 disabled:cursor-not-allowed"
              >
                Create Project
              </button>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}
