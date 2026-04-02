import { Component, inject } from '@angular/core';
import { ActivatedRoute } from '@angular/router';
import { FootballDataService } from '../../services/football-data.service';
import { CompetitionStandingsResponse, StandingGroup } from '../../model/competition-standings';
import { CommonModule } from '@angular/common';
import { MatTableModule } from '@angular/material/table';
import { MatButtonModule } from '@angular/material/button';

@Component({
  selector: 'app-competition-details',
  standalone: true,
  imports: [CommonModule, MatTableModule, MatButtonModule],
  templateUrl: './competition-details.component.html',
  styleUrl: './competition-details.component.css'
})
export class CompetitionDetailsComponent {
private route = inject(ActivatedRoute);
  private footballDataService = inject(FootballDataService);

  data: CompetitionStandingsResponse | null = null;
  selectedStandingType = 'TOTAL';
  loading = false;
  error = '';

  displayedColumns = ['position', 'team', 'played', 'won', 'draw', 'lost', 'gf', 'ga', 'gd', 'points', 'form'];

  ngOnInit(): void {
    const code = this.route.snapshot.paramMap.get('code');

    if (code) {
      this.loadStandings(code);
    }
  }

  loadStandings(code: string): void {
    this.loading = true;
    this.error = '';

    this.footballDataService.getCompetitionStandings(code).subscribe({
      next: (data) => {
        this.data = data;
        this.loading = false;
      },
      error: () => {
        this.error = 'Failed to load standings.';
        this.loading = false;
      }
    });
  }

  get standingTypes(): string[] {
    return this.data?.standings.map(s => s.standing_type) ?? [];
  }

  get selectedStanding(): StandingGroup | undefined {
    return this.data?.standings.find(s => s.standing_type === this.selectedStandingType);
  }

  selectStanding(type: string): void {
    this.selectedStandingType = type;
  }
}
