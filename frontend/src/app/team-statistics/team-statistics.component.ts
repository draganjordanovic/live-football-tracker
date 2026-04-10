import { Component, inject } from '@angular/core';
import { ActivatedRoute } from '@angular/router';
import { CommonModule } from '@angular/common';
import { MatCardModule } from '@angular/material/card';
import { TeamStatisticsService } from '../services/team-statistics.service';
import { TeamStatisticsResponse } from '../model/team-statistics';

@Component({
  selector: 'app-team-statistics',
  standalone: true,
  imports: [CommonModule, MatCardModule],
  templateUrl: './team-statistics.component.html',
  styleUrl: './team-statistics.component.css'
})
export class TeamStatisticsComponent {
  private route = inject(ActivatedRoute);
  private teamStatisticsService = inject(TeamStatisticsService);

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