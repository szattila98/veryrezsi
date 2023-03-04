import type { User as DomainUser } from "$shared/domain";

// See https://kit.svelte.dev/docs/types#app
// for information about these interfaces
declare global {
	namespace App {
		interface Locals {
			user: PublicUser
		}
		// interface Error {}
		// interface PageData {}
		// interface Platform {}
	}
}

export type PublicUser = Omit<DomainUser, 'id'> | undefined | null

// See https://kit.svelte.dev/docs/types#app -
export {};