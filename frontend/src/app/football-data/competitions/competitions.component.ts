import { Component, inject } from '@angular/core';
import { Competition } from '../../model/competition';
import { FootballDataService } from '../../services/football-data.service';
import { CommonModule } from '@angular/common';
import { MatCardModule } from '@angular/material/card';

@Component({
  selector: 'app-competitions',
  standalone: true,
  imports: [CommonModule, MatCardModule],
  templateUrl: './competitions.component.html',
  styleUrl: './competitions.component.css'
})
export class CompetitionsComponent {
  private footballDataService = inject(FootballDataService);

  competitions: Competition[] = [];
  loading = false;
  error = '';

  ngOnInit(): void {
    this.loadCompetitions();
  }

  loadCompetitions(): void {
    this.loading = true;
    this.error = '';

    this.footballDataService.getCompetitions().subscribe({
      next: (data) => {
        this.competitions = data;
        this.loading = false;
      },
      error: () => {
        this.error = 'Failed to load competitions.';
        this.loading = false;
      }
    });
  }
}
