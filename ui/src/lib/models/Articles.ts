import type { Language } from '$lib/models/Language';

export type ArticleData = {
	id: string;
	language: Language,
	data: {
		title: string;
		content: string;
	},
	seq: number;
}

export type CreateArticleRequest = {
	title: string;
	content: string;
	language: Language;
	seq: number;
}

export type UpdateArticleRequest = { id: string } & CreateArticleRequest;