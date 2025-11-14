import { ComponentFixture, TestBed } from '@angular/core/testing';

import { SideButton } from './side-button';

describe('SideButton', () => {
  let component: SideButton;
  let fixture: ComponentFixture<SideButton>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [SideButton]
    })
    .compileComponents();

    fixture = TestBed.createComponent(SideButton);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
