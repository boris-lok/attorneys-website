export type SimpleArticle = {
	id: string;
	title: string;
}

export type Article = SimpleArticle & { content: string };