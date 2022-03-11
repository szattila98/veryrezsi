/* eslint-disable @typescript-eslint/no-empty-interface */
/// <reference types="@sveltejs/kit" />

// See https://kit.svelte.dev/docs#typescript
// for information about these interfaces
declare namespace App {
	interface Locals {
		user: null | {
			id: number;
			email: string;
			username: string;
		}
	}

	interface Platform {}

	interface Session {
		user: {
			id: number;
			email: string;
			username: string;
		}
	}

	interface Stuff {}
}

declare namespace Profile {
	interface User {
		id: number;
		email: string;
		username: string;
	}
}
