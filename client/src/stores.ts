import type { User } from '$shared/domain';

import { writable, type Writable } from 'svelte/store';

// While server determines whether the user is logged in by examining RequestEvent.locals.user, the
// loginSession is updated so all parts of the SPA client-side see the user and role.
export const loginSession: Writable<User | undefined> = writable(undefined);
