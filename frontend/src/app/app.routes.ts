import { Routes } from '@angular/router';
import { CompetitionsComponent } from './football-data/competitions/competitions.component';
import { CompetitionDetailsComponent } from './football-data/competition-details/competition-details.component';
import { MatchDetailsComponent } from './football-data/match-details/match-details.component';
import { TeamStatisticsComponent } from './team-statistics/team-statistics.component';
import { RegisterComponent } from './register/register.component';
import { LoginComponent } from './login/login.component';

export const routes: Routes = [
  { path: '', redirectTo: 'login', pathMatch: 'full' },

  { path: 'login', component: LoginComponent },
  { path: 'register', component: RegisterComponent },

  { path: 'competitions', component: CompetitionsComponent },
  { path: 'competitions/:code', component: CompetitionDetailsComponent },
  { path: 'competitions/:code/teams/:teamId/statistics', component: TeamStatisticsComponent },
  { path: 'matches/:id', component: MatchDetailsComponent },

  { path: '**', redirectTo: 'login' }
];