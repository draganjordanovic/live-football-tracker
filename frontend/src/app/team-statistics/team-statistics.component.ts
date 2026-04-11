import { Component, inject } from '@angular/core';
import { ActivatedRoute } from '@angular/router';
import { CommonModule } from '@angular/common';
import { MatCardModule } from '@angular/material/card';
import { TeamStatisticsService } from '../services/team-statistics.service';
import { TeamStatisticsResponse } from '../model/team-statistics';
import { AuthService } from '../services/auth.service';
import { FavoriteClubsService } from '../services/favorite-clubs.service';
import { MatIconModule } from '@angular/material/icon';
import { MatButtonModule } from '@angular/material/button';

@Component({
  selector: 'app-team-statistics',
  standalone: true,
  imports: [CommonModule, MatCardModule, MatIconModule, MatButtonModule],
  templateUrl: './team-statistics.component.html',
  styleUrl: './team-statistics.component.css'
})
export class TeamStatisticsComponent {
  private route = inject(ActivatedRoute);
  private teamStatisticsService = inject(TeamStatisticsService);

  favoriteClubsService = inject(FavoriteClubsService);
  authService = inject(AuthService);

  isFavorite = false;

  data: TeamStatisticsResponse | null = null;
  loading = false;
  error = '';

  ngOnInit(): void {
    const code = this.route.snapshot.paramMap.get('code');
    const teamId = this.route.snapshot.paramMap.get('teamId');

    if (!code || !teamId) {
      this.error = 'Missing team statistics parameters.';
      return;
    }

    this.loadStatistics(code, Number(teamId));

    if (this.authService.isLoggedIn()) {
      this.loadFavoriteState();
}
  }

  loadStatistics(code: string, teamId: number): void {
    this.loading = true;
    this.error = '';

    this.teamStatisticsService.getTeamStatistics(code, teamId).subscribe({
      next: (response) => {
        this.data = response;
        this.loading = false;
      },
      error: () => {
        this.error = 'Failed to load team statistics.';
        this.loading = false;
      }
    });
  }

  loadFavoriteState(): void {
  const teamId = this.route.snapshot.paramMap.get('teamId');

  if (!teamId || !this.authService.isLoggedIn()) {
    return;
  }

  this.favoriteClubsService.getMyFavoriteClubs().subscribe({
    next: (favorites) => {
      this.isFavorite = favorites.some(f => f.club_external_id === Number(teamId));
    },
    error: () => {
      this.isFavorite = false;
    }
  });
}

toggleFavorite(): void {
  if (!this.data || !this.authService.isLoggedIn()) {
    return;
  }

  if (this.isFavorite) {
    this.favoriteClubsService.removeFavoriteClub(this.data.team_id).subscribe({
      next: () => {
        this.isFavorite = false;
      }
    });
  } else {
    this.favoriteClubsService.addFavoriteClub({
      club_external_id: this.data.team_id,
      competition_code: this.data.competition_code,
      club_name: this.data.team_name,
      club_short_name: this.data.team_short_name,
      club_crest: this.data.team_crest
    }).subscribe({
      next: () => {
        this.isFavorite = true;
      }
    });
  }
}


  percentage(value: number): string {
    return `${(value * 100).toFixed(1)}%`;
  }

  number(value: number): string {
    return value.toFixed(2);
  }

  formItems(): string[] {
  if (!this.data?.form) {
    return [];
  }

  return this.data.form.split(',').map(item => item.trim());
}

downloadPdf(): void {
  const code = this.route.snapshot.paramMap.get('code');
  const teamId = this.route.snapshot.paramMap.get('teamId');

  if (!code || !teamId) {
    return;
  }

  const url = `http://127.0.0.1:3002/reports/competitions/${code}/teams/${teamId}/statistics/pdf`;
  window.location.href = url;
}
}