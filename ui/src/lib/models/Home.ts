import type { Language } from '$lib/models/Language';

export type HomeData = {
	id: string;
	language: Language,
	data: {
		data: string;
	}
}