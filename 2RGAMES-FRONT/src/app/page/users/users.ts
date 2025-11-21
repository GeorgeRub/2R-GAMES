import { Component, inject, OnInit, signal } from '@angular/core';
import { UserService } from '../../services/users/user-service';
import { BooleanPipe } from '../../pipes/boolean-pipe';

@Component({
  selector: 'app-users',
  imports: [BooleanPipe],
  templateUrl: './users.html',
  styleUrl: './users.css',
})
export class Users implements OnInit {
  private user_service = inject(UserService);

  constructor() {}

  users= signal<Array<User>>([]);
  

  ngOnInit(): void {
    console.log('====================================');
    console.log('users init');
    console.log('====================================');
    this.user_service.getUsers().subscribe((data) => {
      console.log('====================================');      
      this.users.set(data as User[]);
      console.log('local users:', this.users);
      console.log('====================================');
    });
  }

}
