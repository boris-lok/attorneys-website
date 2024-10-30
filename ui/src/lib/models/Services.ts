import type { Language } from '$lib/models/Language';

export type ServiceData = {
	id: string;
	language: Language;
	data: {
		title: string;
		data: string;
	},
	seq: number;
}

export type CreateServiceRequest = {
	title: string;
	data: string;
	language: Language;
	seq: number;
}

export type UpdateServiceRequest = { id: string } & CreateServiceRequest;
