import { ComponentFixture, TestBed } from '@angular/core/testing';

import { FavoriteClubsComponent } from './favorite-clubs.component';

describe('FavoriteClubsComponent', () => {
  let component: FavoriteClubsComponent;
  let fixture: ComponentFixture<FavoriteClubsComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [FavoriteClubsComponent]
    })
    .compileComponents();

    fixture = TestBed.createComponent(FavoriteClubsComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
