import type { Language } from '$lib/models/Language';

export type HomeData = {
	id: string;
	language: Language,
	data: {
		data: string;
	}
}

export type CreateHomeRequest = {
	data: string;
	language: Language;
	seq: number;
}

export type UpdateHomeRequest = { id: string } & CreateHomeRequest;