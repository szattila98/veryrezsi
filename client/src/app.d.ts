import type { User } from '$shared/domain';

// See https://kit.svelte.dev/docs/types#app
// for information about these interfaces
declare global {
	namespace App {
		interface Locals {
			user: User;
		}
		// interface Error {}
		// interface PageData {}
		// interface Platform {}
	}
}

// See https://kit.svelte.dev/docs/types#app -
export {};
