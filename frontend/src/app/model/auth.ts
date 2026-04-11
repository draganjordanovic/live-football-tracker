export interface RegisterRequest {
  email: string;
  password: string;
}

export interface LoginRequest {
  email: string;
  password: string;
}

export interface AuthResponse {
  token: string;
}

export interface MeResponse {
  id: string;
  email: string;
  role: string;
  created_at: string;
}