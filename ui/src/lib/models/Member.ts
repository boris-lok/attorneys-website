import type { Language } from '$lib/models/Language';

export type SimpleMember = {
	id: string;
	name: string;
	avatar?: string;
}

export type CreateMemberRequest = {
	name: string;
	description: string;
	language: Language;
}

export type UpdateMemberRequest = { id: string } & CreateMemberRequest;