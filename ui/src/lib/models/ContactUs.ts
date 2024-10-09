import type { Language } from '$lib/models/Language';

export type ContactData = {
	id: string;
	language: Language,
	data: {
		address: string;
		phone: string;
		email: string;
	}
}