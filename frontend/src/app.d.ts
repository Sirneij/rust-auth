// See https://kit.svelte.dev/docs/types#app
// for information about these interfaces
declare global {
	namespace App {
		// interface Error {}
		interface Locals {
			user: {
				email: string;
				first_name: string;
				last_name: string;
				id: string;
				is_staff: boolean;
				thumbnail: string;
				is_superuser: boolean;
			};
		}
		// interface PageData {}
		// interface Platform {}
	}
}

export {};
