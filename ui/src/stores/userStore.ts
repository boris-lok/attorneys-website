import { get, writable } from 'svelte/store';

const createWritableStore = (key: string, initial_value: any) => {
	const { subscribe, set } = writable(initial_value);

	return {
		subscribe,
		set,
		useLocalStorage: () => {
			const json = localStorage.getItem(key);
			if (json && json !== 'null') {
				set(json);
			}

			subscribe(current => {
				localStorage.setItem(key, current);
			});
		},
		remove: () => {
			user.set(null);
			localStorage.removeItem(key);
		},
		get: () => {
			return get(user);
		}
	};
};

export const user = createWritableStore('user', null);