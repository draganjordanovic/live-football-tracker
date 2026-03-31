import { inject, Injectable } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { Observable } from 'rxjs';
import { Competition } from '../model/competition';

@Injectable({
  providedIn: 'root'
})
export class FootballDataService {

  private http = inject(HttpClient);
  private baseUrl = 'http://127.0.0.1:3000';

  getCompetitions(): Observable<Competition[]> {
    return this.http.get<Competition[]>(`${this.baseUrl}/competitions`);
  }
}
