import { inject, Injectable } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { Observable } from 'rxjs';
import { AuthService } from './auth.service';

export interface FavoriteClub {
  id: number;
  user_id: string;
  club_external_id: number;
  competition_code: string;
  club_name: string;
  club_short_name: string | null;
  club_crest: string | null;
  created_at: string;
}

export interface CreateFavoriteClubRequest {
  club_external_id: number;
  competition_code: string;
  club_name: string;
  club_short_name: string | null;
  club_crest: string | null;
}

@Injectable({
  providedIn: 'root'
})
export class FavoriteClubsService {
  private http = inject(HttpClient);
  private authService = inject(AuthService);
  private baseUrl = 'http://127.0.0.1:3003';

  getMyFavoriteClubs(): Observable<FavoriteClub[]> {
    return this.http.get<FavoriteClub[]>(
      `${this.baseUrl}/users/me/favorite-clubs`,
      { headers: this.authService.getAuthHeaders() }
    );
  }

  addFavoriteClub(payload: CreateFavoriteClubRequest): Observable<FavoriteClub> {
    return this.http.post<FavoriteClub>(
      `${this.baseUrl}/users/me/favorite-clubs`,
      payload,
      { headers: this.authService.getAuthHeaders() }
    );
  }

  removeFavoriteClub(clubId: number): Observable<void> {
    return this.http.delete<void>(
      `${this.baseUrl}/users/me/favorite-clubs/${clubId}`,
      { headers: this.authService.getAuthHeaders() }
    );
  }
}