import { AfterViewInit, Component, ElementRef, OnInit, output, signal, ViewChild } from '@angular/core';
import { SideButton } from "./side-button/side-button";
import { NgIcon, provideIcons } from "@ng-icons/core";
import { ionMenu } from "@ng-icons/ionicons";

@Component({
  selector: 'app-sidebar',
  imports: [SideButton, NgIcon],
  viewProviders: [provideIcons({ionMenu})],
  templateUrl: './sidebar.html',
  styleUrl: './sidebar.css',
})
export class Sidebar implements AfterViewInit {

  @ViewChild('sidebar') divSidebar!: ElementRef<HTMLDivElement>;

  showSidebar = signal(false);

  ngAfterViewInit(): void {
    console.log('====================================');
    console.log(this.divSidebar);
    console.log('====================================');
    this.changeSidebarVisibility();
  }

  openedMenu = 'w-82'
  closedMenu = 'w-16'

  changeSidebarVisibility() {
    console.log('Toggling sidebar visibility');
    // this.showSidebar = !this.showSidebar;
    if (this.showSidebar()) {
      this.divSidebar.nativeElement.classList.add(this.openedMenu);
      this.divSidebar.nativeElement.classList.remove(this.closedMenu);
    } else {
      this.divSidebar.nativeElement.classList.add(this.closedMenu);
      this.divSidebar.nativeElement.classList.remove(this.openedMenu);
    }
  }

  swapSidebarVisibility() {
    this.showSidebar.set(!this.showSidebar());
    this.changeSidebarVisibility();
  }

}
