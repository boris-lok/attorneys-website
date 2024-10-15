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

export type MemberData = {
	id: string;
	language: Language,
	data: {
		name: string;
		description: string;
	}
	avatar?: {
		large_image: string;
		small_image: string;
	}
}

export type UpdateMemberRequest = { id: string } & CreateMemberRequest;