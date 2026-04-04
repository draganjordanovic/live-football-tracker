import { Component, inject } from '@angular/core';
import { ActivatedRoute } from '@angular/router';
import { CommonModule, DatePipe } from '@angular/common';
import { MatCardModule } from '@angular/material/card';
import { MatDividerModule } from '@angular/material/divider';
import { FootballDataService } from '../../services/football-data.service';
import { MatchDetailsResponse, GoalItem, BookingItem } from '../../model/match-details';

@Component({
  selector: 'app-match-details',
  standalone: true,
  imports: [CommonModule, DatePipe, MatCardModule, MatDividerModule],
  templateUrl: './match-details.component.html',
  styleUrl: './match-details.component.css'
})
export class MatchDetailsComponent {
  private route = inject(ActivatedRoute);
  private footballDataService = inject(FootballDataService);

  data: MatchDetailsResponse | null = null;
  loading = false;
  error = '';

  ngOnInit(): void {
    const id = this.route.snapshot.paramMap.get('id');

    if (!id) {
      this.error = 'Match id is missing.';
      return;
    }

    this.loadMatchDetails(Number(id));
  }

  loadMatchDetails(id: number): void {
    this.loading = true;
    this.error = '';

    this.footballDataService.getMatchDetails(id).subscribe({
      next: (response) => {
        this.data = response;
        this.loading = false;
      },
      error: () => {
        this.error = 'Failed to load match details.';
        this.loading = false;
      }
    });
  }

  get fullTimeScore(): string {
    if (!this.data) return '- : -';

    const home = this.data.score.full_time.home;
    const away = this.data.score.full_time.away;

    if (home === null || away === null) {
      return '- : -';
    }

    return `${home} : ${away}`;
  }

  get halfTimeScore(): string {
    if (!this.data) return '- : -';

    const home = this.data.score.half_time.home;
    const away = this.data.score.half_time.away;

    if (home === null || away === null) {
      return '- : -';
    }

    return `${home} : ${away}`;
  }

  get hasEventData(): boolean {
    console.log(this.data)
    return !!this.data && (this.data.goals.length > 0 || this.data.bookings.length > 0);
  }

  formatStatus(status: string): string {
    switch (status) {
      case 'TIMED':
        return 'Scheduled';
      case 'IN_PLAY':
        return 'Live';
      case 'PAUSED':
        return 'Paused';
      case 'FINISHED':
        return 'Finished';
      case 'POSTPONED':
        return 'Postponed';
      case 'SUSPENDED':
        return 'Suspended';
      case 'CANCELLED':
        return 'Cancelled';
      default:
        return status;
    }
  }

  formatMinute(item: GoalItem | BookingItem): string {
    if (item.minute === null) return '-';
    if ('injury_time' in item && item.injury_time) {
      return `${item.minute}+${item.injury_time}'`;
    }
    return `${item.minute}'`;
  }

  isFinishedOrLive(): boolean {
    if (!this.data) return false;

    return ['IN_PLAY', 'PAUSED', 'FINISHED'].includes(this.data.status);
  }
}