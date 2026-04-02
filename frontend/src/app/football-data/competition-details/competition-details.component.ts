import { Component, inject } from '@angular/core';
import { ActivatedRoute, Router } from '@angular/router';
import { FootballDataService } from '../../services/football-data.service';
import { CompetitionStandingsResponse, StandingGroup } from '../../model/competition-standings';
import { CompetitionMatchesResponse, MatchItem } from '../../model/competition-matches';
import { CommonModule, DatePipe } from '@angular/common';
import { MatTableModule } from '@angular/material/table';
import { MatButtonModule } from '@angular/material/button';
import { MatCardModule } from '@angular/material/card';

@Component({
  selector: 'app-competition-details',
  standalone: true,
  imports: [CommonModule, DatePipe, MatTableModule, MatButtonModule, MatCardModule],
  templateUrl: './competition-details.component.html',
  styleUrl: './competition-details.component.css'
})
export class CompetitionDetailsComponent {
  private route = inject(ActivatedRoute);
  private router = inject(Router);
  private footballDataService = inject(FootballDataService);

  data: CompetitionStandingsResponse | null = null;
  matchesData: CompetitionMatchesResponse | null = null;

  selectedStandingType = 'TOTAL';
  loading = false;
  error = '';

  displayedColumns = ['position', 'team', 'played', 'won', 'draw', 'lost', 'gf', 'ga', 'gd', 'points', 'form'];

  ngOnInit(): void {
    const code = this.route.snapshot.paramMap.get('code');

    if (code) {
      this.loadCompetitionPage(code);
    }
  }

  loadCompetitionPage(code: string): void {
    this.loading = true;
    this.error = '';

    this.footballDataService.getCompetitionStandings(code).subscribe({
      next: (standingsData) => {
        this.data = standingsData;

        const currentMatchday = standingsData.season.current_matchday;

        if (currentMatchday === null) {
          this.matchesData = { competition: standingsData.competition, matches: [] };
          this.loading = false;
          return;
        }

        this.footballDataService.getCompetitionMatches(code, currentMatchday).subscribe({
          next: (matchesData) => {
            this.matchesData = matchesData;
            this.loading = false;
          },
          error: () => {
            this.error = 'Failed to load current matchday matches.';
            this.loading = false;
          }
        });
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

  get currentMatchday(): number | null {
    return this.data?.season.current_matchday ?? null;
  }

  get currentMatchdayMatches(): MatchItem[] {
    return this.matchesData?.matches ?? [];
  }

  selectStanding(type: string): void {
    this.selectedStandingType = type;
  }

  getScore(match: MatchItem): string {
    if (match.score.home === null || match.score.away === null) {
      return '- : -';
    }

    return `${match.score.home} : ${match.score.away}`;
  }

  openMatchDetails(matchId: number): void {
  this.router.navigate(['/matches', matchId]);
}
}