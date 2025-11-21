import { Routes } from '@angular/router';
import { Index } from './page/index';
import { Users } from './page/users/users';

export const routes: Routes = [
    {path:'', component: Index},
    {path:'users', component: Users},
    {path:'admin', loadChildren: () => import('./page/admin/admin-module').then(m => m.AdminModule) }
];
