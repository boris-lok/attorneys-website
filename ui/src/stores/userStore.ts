import { get, writable } from 'svelte/store';
import Cookies from 'js-cookie';
import type { LoginResponse } from '$lib/models/User';

const createWritableStore = (key: string, initial_value: any) => {
	const { subscribe, set } = writable(initial_value);

	return {
		subscribe,
		set,
		useLocalStorage: () => {
			const u = Cookies.get(key);
			if (u && u !== '') {
				set(JSON.parse(u) as LoginResponse);
			}

			subscribe(current => {
				Cookies.set(key, JSON.stringify(current), { expires: 7 });
			});
		},
		remove: () => {
			user.set(null);
			Cookies.remove(key);
		},
		get: () => {
			return get(user);
		}
	};
};

export const user = createWritableStore('user', null);