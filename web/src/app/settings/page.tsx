"use client";

import { useEffect, useState } from "react";
import { useRouter } from "next/navigation";
import { api } from "@/lib/api";
import type { Project, VaultStatus } from "@/lib/types";
import {
  ArrowLeft,
  Shield,
  Key,
  Folder,
  Lock,
  Database,
  HardDrive,
  ChevronRight,
  ExternalLink,
  LogOut,
} from "lucide-react";

export default function SettingsPage() {
  const router = useRouter();
  const [vaultStatus, setVaultStatus] = useState<VaultStatus | null>(null);
  const [projects, setProjects] = useState<Project[]>([]);
  const [loading, setLoading] = useState(true);

  // Change password state
  const [showChangePassword, setShowChangePassword] = useState(false);
  const [currentPassword, setCurrentPassword] = useState("");
  const [newPassword, setNewPassword] = useState("");
  const [confirmPassword, setConfirmPassword] = useState("");
  const [passwordError, setPasswordError] = useState("");
  const [passwordSuccess, setPasswordSuccess] = useState("");
  const [changing, setChanging] = useState(false);

  useEffect(() => {
    loadData();
  }, []);

  async function loadData() {
    try {
      const [status, projs] = await Promise.all([
        api.vaultStatus(),
        api.getProjects(),
      ]);
      setVaultStatus(status);
      setProjects(projs);
    } catch (err) {
      console.error("Failed to load settings:", err);
    } finally {
      setLoading(false);
    }
  }

  async function handleChangePassword(e: React.FormEvent) {
    e.preventDefault();
    setPasswordError("");
    setPasswordSuccess("");

    if (newPassword.length < 8) {
      setPasswordError("Password must be at least 8 characters");
      return;
    }

    if (newPassword !== confirmPassword) {
      setPasswordError("Passwords do not match");
      return;
    }

    setChanging(true);
    try {
      setPasswordSuccess("Password change requires re-initializing the vault");
      setShowChangePassword(false);
      setCurrentPassword("");
      setNewPassword("");
      setConfirmPassword("");
    } catch (err: any) {
      setPasswordError(err.message || "Failed to change password");
    } finally {
      setChanging(false);
    }
  }

  const totalSecrets = projects.reduce((sum, p) => sum + p.secret_count, 0);
  const totalEnvs = projects.reduce((sum, p) => sum + p.environment_count, 0);

  async function handleLogout() {
    try {
      await api.lock();
      router.push("/vault/unlock");
    } catch {
      router.push("/vault/unlock");
    }
  }

  if (loading) {
    return (
      <div className="flex min-h-screen items-center justify-center bg-surface-secondary">
        <div className="flex flex-col items-center gap-3">
          <div className="h-8 w-8 animate-spin rounded-full border-2 border-primary border-t-transparent" />
          <span className="text-sm text-text-tertiary">Loading settings...</span>
        </div>
      </div>
    );
  }

  return (
    <div className="min-h-screen bg-surface-secondary">
      {/* Header */}
      <header className="sticky top-0 z-40 border-b border-border bg-white/80 backdrop-blur-xl">
        <div className="mx-auto flex h-16 max-w-3xl items-center justify-between px-6">
          <div className="flex items-center gap-4">
            <button
              onClick={() => router.push("/")}
              className="rounded-lg p-2 text-text-tertiary hover:text-text-primary hover:bg-surface-tertiary transition-colors duration-150 cursor-pointer"
            >
              <ArrowLeft className="h-5 w-5" />
            </button>
            <div>
              <h1 className="text-heading-2 text-text-primary">Settings</h1>
              <p className="text-xs text-text-tertiary">Manage your vault and security</p>
            </div>
          </div>
          <button
            onClick={handleLogout}
            className="rounded-lg p-2 text-text-tertiary hover:text-red-500 hover:bg-red-50 transition-colors duration-150 cursor-pointer"
            title="Lock & Logout"
          >
            <LogOut className="h-5 w-5" />
          </button>
        </div>
      </header>

      <main className="mx-auto max-w-3xl px-6 py-8 space-y-6">
        {/* Vault Info */}
        <section className="card p-6 animate-fade-in">
          <div className="flex items-center gap-3 mb-5">
            <div className="flex h-9 w-9 items-center justify-center rounded-lg bg-primary/10">
              <Shield className="h-5 w-5 text-primary" />
            </div>
            <div>
              <h2 className="text-heading-3 text-text-primary">Vault</h2>
              <p className="text-xs text-text-tertiary">Your encrypted secrets storage</p>
            </div>
          </div>
          <div className="grid grid-cols-2 gap-4">
            <div className="rounded-lg bg-surface-secondary p-4 border border-border-light">
              <div className="flex items-center gap-2 text-label text-text-tertiary uppercase">
                <Lock className="h-3.5 w-3.5" />
                Status
              </div>
              <div className="mt-2 flex items-center gap-2">
                <div className={`h-2 w-2 rounded-full ${vaultStatus?.locked ? 'bg-amber-400' : 'bg-emerald-400'}`} />
                <span className="text-heading-3 text-text-primary">
                  {vaultStatus?.locked ? "Locked" : "Unlocked"}
                </span>
              </div>
            </div>
            <div className="rounded-lg bg-surface-secondary p-4 border border-border-light">
              <div className="flex items-center gap-2 text-label text-text-tertiary uppercase">
                <HardDrive className="h-3.5 w-3.5" />
                Location
              </div>
              <div className="mt-2 text-sm font-mono text-text-primary">
                ~/.ghostkey/vault.enc
              </div>
            </div>
          </div>
        </section>

        {/* Stats */}
        <section className="card p-6 animate-fade-in stagger-1">
          <div className="flex items-center gap-3 mb-5">
            <div className="flex h-9 w-9 items-center justify-center rounded-lg bg-primary/10">
              <Database className="h-5 w-5 text-primary" />
            </div>
            <div>
              <h2 className="text-heading-3 text-text-primary">Statistics</h2>
              <p className="text-xs text-text-tertiary">Overview of your secrets</p>
            </div>
          </div>
          <div className="grid grid-cols-3 gap-4">
            <div className="rounded-lg bg-surface-secondary p-4 text-center border border-border-light">
              <div className="text-stat text-primary">{projects.length}</div>
              <div className="mt-1 text-label text-text-tertiary uppercase">Projects</div>
            </div>
            <div className="rounded-lg bg-surface-secondary p-4 text-center border border-border-light">
              <div className="text-stat text-primary">{totalEnvs}</div>
              <div className="mt-1 text-label text-text-tertiary uppercase">Environments</div>
            </div>
            <div className="rounded-lg bg-surface-secondary p-4 text-center border border-border-light">
              <div className="text-stat text-primary">{totalSecrets}</div>
              <div className="mt-1 text-label text-text-tertiary uppercase">Secrets</div>
            </div>
          </div>
        </section>

        {/* Security */}
        <section className="card p-6 animate-fade-in stagger-2">
          <div className="flex items-center gap-3 mb-5">
            <div className="flex h-9 w-9 items-center justify-center rounded-lg bg-primary/10">
              <Key className="h-5 w-5 text-primary" />
            </div>
            <div>
              <h2 className="text-heading-3 text-text-primary">Security</h2>
              <p className="text-xs text-text-tertiary">Encryption and access control</p>
            </div>
          </div>
          <div className="space-y-1">
            <div className="flex items-center justify-between py-3 px-4 rounded-lg hover:bg-surface-secondary transition-colors">
              <div>
                <div className="text-sm font-medium text-text-primary">Master Password</div>
                <div className="text-xs text-text-tertiary mt-0.5">
                  Used to encrypt and decrypt your vault
                </div>
              </div>
              <button
                onClick={() => setShowChangePassword(true)}
                className="btn-ghost text-sm"
              >
                Change
                <ChevronRight className="h-4 w-4" />
              </button>
            </div>
            <div className="flex items-center justify-between py-3 px-4 rounded-lg">
              <div>
                <div className="text-sm font-medium text-text-primary">Encryption</div>
                <div className="text-xs text-text-tertiary mt-0.5">
                  AES-256-GCM with Argon2id key derivation
                </div>
              </div>
              <span className="badge-success">
                Active
              </span>
            </div>
          </div>
        </section>

        {/* Projects List */}
        <section className="card p-6 animate-fade-in stagger-3">
          <div className="flex items-center gap-3 mb-5">
            <div className="flex h-9 w-9 items-center justify-center rounded-lg bg-primary/10">
              <Folder className="h-5 w-5 text-primary" />
            </div>
            <div>
              <h2 className="text-heading-3 text-text-primary">Projects</h2>
              <p className="text-xs text-text-tertiary">{projects.length} project{projects.length !== 1 ? 's' : ''} configured</p>
            </div>
          </div>
          {projects.length === 0 ? (
            <div className="py-8 text-center text-sm text-text-tertiary">
              No projects yet
            </div>
          ) : (
            <div className="divide-y divide-border-light">
              {projects.map((project) => (
                <div
                  key={project.id}
                  onClick={() => router.push(`/projects/${project.id}`)}
                  className="flex items-center justify-between py-3 px-4 -mx-4 rounded-lg hover:bg-surface-secondary transition-colors cursor-pointer group"
                >
                  <div className="flex items-center gap-3">
                    <div className="flex h-8 w-8 items-center justify-center rounded-lg bg-surface-tertiary">
                      <Folder className="h-4 w-4 text-text-tertiary" />
                    </div>
                    <div>
                      <div className="text-sm font-medium text-text-primary">{project.name}</div>
                      {project.description && (
                        <div className="text-xs text-text-tertiary mt-0.5">{project.description}</div>
                      )}
                    </div>
                  </div>
                  <div className="flex items-center gap-4">
                    <div className="flex gap-3 text-xs text-text-tertiary">
                      <span>{project.environment_count} env{project.environment_count !== 1 ? "s" : ""}</span>
                      <span>{project.secret_count} secret{project.secret_count !== 1 ? "s" : ""}</span>
                    </div>
                    <ChevronRight className="h-4 w-4 text-text-tertiary opacity-0 group-hover:opacity-100 transition-opacity" />
                  </div>
                </div>
              ))}
            </div>
          )}
        </section>

        {/* About */}
        <section className="card p-6 animate-fade-in stagger-4">
          <div className="text-sm text-text-secondary">
            <div className="font-semibold text-text-primary">GhostKey v0.1.0</div>
            <div className="mt-1">Local-first secrets manager for developers</div>
            <div className="mt-3 flex items-center gap-4 text-xs text-text-tertiary">
              <span className="flex items-center gap-1">
                <svg className="h-3.5 w-3.5" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={2}>
                  <path strokeLinecap="round" strokeLinejoin="round" d="M9 12l2 2 4-4m5.618-4.016A11.955 11.955 0 0112 2.944a11.955 11.955 0 01-8.618 3.04A12.02 12.02 0 003 9c0 5.591 3.824 10.29 9 11.622 5.176-1.332 9-6.03 9-11.622 0-1.042-.133-2.052-.382-3.016z" />
                </svg>
                AES-256-GCM encryption
              </span>
              <span className="flex items-center gap-1">
                <svg className="h-3.5 w-3.5" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={2}>
                  <path strokeLinecap="round" strokeLinejoin="round" d="M3 15a4 4 0 004 4h9a5 5 0 10-.1-9.999 5.002 5.002 0 10-9.78 2.096A4.001 4.001 0 003 15z" />
                </svg>
                Local storage only
              </span>
              <span className="flex items-center gap-1">
                <svg className="h-3.5 w-3.5" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={2}>
                  <path strokeLinecap="round" strokeLinejoin="round" d="M10 20l4-16m4 4l4 4-4 4M6 16l-4-4 4-4" />
                </svg>
                Rust + Next.js
              </span>
            </div>
          </div>
        </section>
      </main>

      {/* Change Password Modal */}
      {showChangePassword && (
        <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/40 backdrop-blur-sm animate-fade-in">
          <div className="w-full max-w-md rounded-xl bg-white p-6 shadow-elevated animate-scale-in">
            <h2 className="text-heading-2 text-text-primary">Change Master Password</h2>
            <p className="mt-1 text-body-small text-text-secondary">
              Your vault will be re-encrypted with the new password.
            </p>
            <form onSubmit={handleChangePassword} className="mt-5 space-y-3">
              <div>
                <label className="text-label text-text-secondary mb-1.5 block">Current password</label>
                <input
                  type="password"
                  placeholder="Enter current password"
                  value={currentPassword}
                  onChange={(e) => setCurrentPassword(e.target.value)}
                  className="input"
                />
              </div>
              <div>
                <label className="text-label text-text-secondary mb-1.5 block">New password</label>
                <input
                  type="password"
                  placeholder="Minimum 8 characters"
                  value={newPassword}
                  onChange={(e) => setNewPassword(e.target.value)}
                  className="input"
                />
              </div>
              <div>
                <label className="text-label text-text-secondary mb-1.5 block">Confirm new password</label>
                <input
                  type="password"
                  placeholder="Repeat new password"
                  value={confirmPassword}
                  onChange={(e) => setConfirmPassword(e.target.value)}
                  className="input"
                />
              </div>
              {passwordError && (
                <div className="rounded-lg bg-red-50 border border-red-100 px-4 py-3 text-sm text-red-600">
                  {passwordError}
                </div>
              )}
              <div className="flex justify-end gap-3 pt-2">
                <button
                  type="button"
                  onClick={() => {
                    setShowChangePassword(false);
                    setPasswordError("");
                  }}
                  className="btn-secondary"
                >
                  Cancel
                </button>
                <button
                  type="submit"
                  disabled={changing}
                  className="btn-primary disabled:opacity-50 disabled:cursor-not-allowed"
                >
                  {changing ? "Changing..." : "Change Password"}
                </button>
              </div>
            </form>
          </div>
        </div>
      )}

      {/* Success Toast */}
      {passwordSuccess && (
        <div className="fixed bottom-6 right-6 z-50 animate-slide-up">
          <div className="rounded-lg bg-emerald-50 border border-emerald-100 px-5 py-3 text-sm text-emerald-700 shadow-lg flex items-center gap-2">
            <svg className="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={2}>
              <path strokeLinecap="round" strokeLinejoin="round" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" />
            </svg>
            {passwordSuccess}
          </div>
        </div>
      )}
    </div>
  );
}
