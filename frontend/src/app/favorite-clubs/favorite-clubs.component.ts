import { Component, inject } from '@angular/core';
import { CommonModule } from '@angular/common';
import { Router } from '@angular/router';
import { MatCardModule } from '@angular/material/card';
import { MatButtonModule } from '@angular/material/button';
import { FavoriteClub, FavoriteClubsService } from '../services/favorite-clubs.service';

@Component({
  selector: 'app-favorite-clubs',
  standalone: true,
  imports: [
    CommonModule,
    MatCardModule,
    MatButtonModule
  ],
  templateUrl: './favorite-clubs.component.html',
  styleUrl: './favorite-clubs.component.css'
})
export class FavoriteClubsComponent {
  private favoriteClubsService = inject(FavoriteClubsService);
  private router = inject(Router);

  favorites: FavoriteClub[] = [];
  loading = false;
  error = '';

  ngOnInit(): void {
    this.loadFavorites();
  }

  loadFavorites(): void {
    this.loading = true;
    this.error = '';

    this.favoriteClubsService.getMyFavoriteClubs().subscribe({
      next: (response) => {
        this.favorites = response;
        this.loading = false;
      },
      error: () => {
        this.error = 'Failed to load favorite clubs.';
        this.loading = false;
      }
    });
  }

  openClubStatistics(club: FavoriteClub): void {
    this.router.navigate([
      '/competitions',
      club.competition_code,
      'teams',
      club.club_external_id,
      'statistics'
    ]);
  }

  removeFavorite(club: FavoriteClub, event: MouseEvent): void {
    event.stopPropagation();

    this.favoriteClubsService.removeFavoriteClub(club.club_external_id).subscribe({
      next: () => {
        this.favorites = this.favorites.filter(
          f => f.club_external_id !== club.club_external_id
        );
      },
      error: () => {
        this.error = 'Failed to remove favorite club.';
      }
    });
  }
}