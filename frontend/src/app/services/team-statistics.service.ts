import { inject, Injectable } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { Observable } from 'rxjs';
import { TeamStatisticsResponse } from '../model/team-statistics';

@Injectable({
  providedIn: 'root'
})
export class TeamStatisticsService {
  private http = inject(HttpClient);
  private baseUrl = 'http://127.0.0.1:3001';

  getTeamStatistics(code: string, teamId: number): Observable<TeamStatisticsResponse> {
    return this.http.get<TeamStatisticsResponse>(
      `${this.baseUrl}/competitions/${code}/teams/${teamId}/statistics`
    );
  }
}