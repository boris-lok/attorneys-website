import type { Language } from '$lib/models/Language';

export type ArticleData = {
	id: string;
	language: Language,
	data: {
		title: string;
		content: string;
	}
}

export type CreateArticleRequest = {
	title: string;
	content: string;
	language: Language;
}

export type UpdateArticleRequest = { id: string } & CreateArticleRequest;