export interface Project {
  id: string;
  name: string;
  description: string | null;
  environment_count: number;
  secret_count: number;
  created_at: string;
  updated_at: string;
}

export interface Environment {
  id: string;
  project_id: string;
  name: string;
  secret_count: number;
  created_at: string;
}

export interface Secret {
  id: string;
  environment_id: string;
  key: string;
  masked: boolean;
  description: string | null;
  created_at: string;
  updated_at: string;
}

export interface SecretValue {
  id: string;
  key: string;
  value: string;
}

export interface VaultStatus {
  initialized: boolean;
  locked: boolean;
}

export interface MessageResponse {
  message: string;
}

export interface ErrorResponse {
  error: string;
}
