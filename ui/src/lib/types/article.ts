import type { BaseData, Language, WithId } from '$lib/types/common'

export type ArticleData = BaseData<{
    title: string
    content: string
    categoryId?: string
}> & {
    seq: number
}

export type SimpleArticle = {
    id: string
    title: string
    language: Language
    createdAt: Date
    seq: number
}

export type CreateArticleRequest = {
    title: string
    content: string
    language: Language
    seq: number
}

export type UpdateArticleRequest = WithId<CreateArticleRequest>
