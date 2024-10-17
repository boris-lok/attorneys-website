import { writable } from 'svelte/store';
import { timer } from 'rxjs';

export const notification = writable({
	message: '',
	type: '' as 'info' | 'error',
	visible: false
});

export function showNotification(message: string, type: 'info' | 'error', timeout: 3000) {
	notification.set({ message: message, type: type, visible: true });

	timer(timeout)
		.subscribe(() => {
			notification.set({ message: '', type: 'info', visible: false });
		});
}