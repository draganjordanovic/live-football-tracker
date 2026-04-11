import { inject, Injectable } from '@angular/core';
import { HttpClient, HttpHeaders } from '@angular/common/http';
import { Observable, tap } from 'rxjs';
import {
  AuthResponse,
  LoginRequest,
  MeResponse,
  RegisterRequest
} from '../model/auth';

@Injectable({
  providedIn: 'root'
})
export class AuthService {
  private http = inject(HttpClient);
  private baseUrl = 'http://127.0.0.1:3003';
  private tokenKey = 'auth_token';

  register(payload: RegisterRequest): Observable<unknown> {
    return this.http.post(`${this.baseUrl}/auth/register`, payload);
  }

  login(payload: LoginRequest): Observable<AuthResponse> {
    return this.http.post<AuthResponse>(`${this.baseUrl}/auth/login`, payload).pipe(
      tap(response => this.setToken(response.token))
    );
  }

  getMe(): Observable<MeResponse> {
    return this.http.get<MeResponse>(`${this.baseUrl}/users/me`, {
      headers: this.getAuthHeaders()
    });
  }

  setToken(token: string): void {
    localStorage.setItem(this.tokenKey, token);
  }

  getToken(): string | null {
    return localStorage.getItem(this.tokenKey);
  }

  clearToken(): void {
    localStorage.removeItem(this.tokenKey);
  }

  isLoggedIn(): boolean {
    return !!this.getToken();
  }

  logout(): void {
    this.clearToken();
  }

  getAuthHeaders(): HttpHeaders {
    const token = this.getToken();

    return new HttpHeaders({
      Authorization: `Bearer ${token ?? ''}`
    });
  }
}