import { TestBed } from '@angular/core/testing';

import { TeamStatisticsService } from './team-statistics.service';

describe('TeamStatisticsService', () => {
  let service: TeamStatisticsService;

  beforeEach(() => {
    TestBed.configureTestingModule({});
    service = TestBed.inject(TeamStatisticsService);
  });

  it('should be created', () => {
    expect(service).toBeTruthy();
  });
});
