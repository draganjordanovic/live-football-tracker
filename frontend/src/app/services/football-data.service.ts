import { inject, Injectable } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { Observable } from 'rxjs';
import { Competition } from '../model/competition';
import { CompetitionStandingsResponse } from '../model/competition-standings';
import { CompetitionMatchesResponse } from '../model/competition-matches';

@Injectable({
  providedIn: 'root'
})
export class FootballDataService {
  private http = inject(HttpClient);
  private baseUrl = 'http://127.0.0.1:3000';

  getCompetitions(): Observable<Competition[]> {
    return this.http.get<Competition[]>(`${this.baseUrl}/competitions`);
  }

  getCompetitionStandings(code: string): Observable<CompetitionStandingsResponse> {
    return this.http.get<CompetitionStandingsResponse>(
      `${this.baseUrl}/competitions/${code}/standings`
    );
  }

  getCompetitionMatches(code: string, matchday: number): Observable<CompetitionMatchesResponse> {
    return this.http.get<CompetitionMatchesResponse>(
      `${this.baseUrl}/competitions/${code}/matches?matchday=${matchday}`
    );
  }
}