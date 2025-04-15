// +layout.ts
import { browser } from '$app/environment';
import { user } from '$lib/stores/user.store';

/** @type {import('./$types').LayoutServerLoad} */
export async function load() {
	if (browser) {
		user.useLocalStorage();
	}
}
