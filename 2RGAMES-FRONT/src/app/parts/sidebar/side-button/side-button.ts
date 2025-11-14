import { Component, input } from '@angular/core';
import { NgIcon, provideIcons } from '@ng-icons/core';
import { featherAirplay } from '@ng-icons/feather-icons';
import { heroUsers } from '@ng-icons/heroicons/outline';
import { ionAccessibilityOutline } from '@ng-icons/ionicons';

@Component({
  selector: 'side-button',
  imports: [NgIcon],
  templateUrl: './side-button.html',
  styleUrl: './side-button.css',
  viewProviders: [provideIcons({ionAccessibilityOutline, featherAirplay, heroUsers })]
})
export class SideButton {

  isOpenedMenu = input.required<boolean>();


}
