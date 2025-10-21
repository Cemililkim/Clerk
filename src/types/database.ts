// Database type definitions for frontend

export interface Project {
  id?: number;
  name: string;
  description?: string;
  created_at: number;
  updated_at: number;
}

export interface Environment {
  id?: number;
  project_id: number;
  name: string;
  description?: string;
  created_at: number;
  updated_at: number;
}

export interface Variable {
  id: number;
  environment_id: number;
  key: string;
  value: string; // Decrypted value
  description?: string;
  created_at: number;
  updated_at: number;
}

// Request/Response types for Tauri commands

// Projects
export interface CreateProjectRequest {
  name: string;
  description?: string;
}

export interface CreateProjectResponse {
  success: boolean;
  project_id?: number;
  message: string;
}

export interface GetProjectsResponse {
  success: boolean;
  projects: Project[];
  message: string;
}

export interface UpdateProjectRequest {
  id: number;
  name: string;
  description?: string;
}

export interface UpdateProjectResponse {
  success: boolean;
  message: string;
}

export interface DeleteProjectRequest {
  id: number;
}

export interface DeleteProjectResponse {
  success: boolean;
  message: string;
}

// Environments
export interface CreateEnvironmentRequest {
  project_id: number;
  name: string;
  description?: string;
}

export interface CreateEnvironmentResponse {
  success: boolean;
  environment_id?: number;
  message: string;
}

export interface GetEnvironmentsRequest {
  project_id: number;
}

export interface GetEnvironmentsResponse {
  success: boolean;
  environments: Environment[];
  message: string;
}

export interface UpdateEnvironmentRequest {
  id: number;
  project_id: number;
  name: string;
  description?: string;
}

export interface UpdateEnvironmentResponse {
  success: boolean;
  message: string;
}

export interface DeleteEnvironmentRequest {
  id: number;
}

export interface DeleteEnvironmentResponse {
  success: boolean;
  message: string;
}

// Variables
export interface CreateVariableRequest {
  environment_id: number;
  key: string;
  value: string;
  description?: string;
}

export interface CreateVariableResponse {
  success: boolean;
  variable_id?: number;
  message: string;
}

export interface GetVariablesRequest {
  environment_id: number;
}

export interface GetVariablesResponse {
  success: boolean;
  variables: Variable[];
  message: string;
}

export interface UpdateVariableRequest {
  id: number;
  key: string;
  value: string;
  description?: string;
}

export interface UpdateVariableResponse {
  success: boolean;
  message: string;
}

export interface DeleteVariableRequest {
  id: number;
}

export interface DeleteVariableResponse {
  success: boolean;
  message: string;
}

// Export/Import
export interface ExportEnvRequest {
  environment_id: number;
}

export interface ExportEnvResponse {
  success: boolean;
  content?: string;
  message: string;
}

export interface ImportEnvRequest {
  environment_id: number;
  content: string;
}

export interface ImportEnvResponse {
  success: boolean;
  imported_count?: number;
  errors?: string[];
  message: string;
}
