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

export type CreateContactRequest = {
	address: string;
	phone: string;
	email: string;
	language: Language;
}

export type UpdateContactRequest = { id: string } & CreateContactRequest;