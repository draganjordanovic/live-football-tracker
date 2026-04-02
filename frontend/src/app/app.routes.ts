import { Routes } from '@angular/router';
import { CompetitionsComponent } from './football-data/competitions/competitions.component';
import { CompetitionDetailsComponent } from './football-data/competition-details/competition-details.component';

export const routes: Routes = [
    {
    path: '',
    component: CompetitionsComponent
  },
  {
    path: 'competitions/:code',
    component: CompetitionDetailsComponent
  }
];
