import { TestBed } from '@angular/core/testing';

import { FavoriteClubsService } from './favorite-clubs.service';

describe('FavoriteClubsService', () => {
  let service: FavoriteClubsService;

  beforeEach(() => {
    TestBed.configureTestingModule({});
    service = TestBed.inject(FavoriteClubsService);
  });

  it('should be created', () => {
    expect(service).toBeTruthy();
  });
});
