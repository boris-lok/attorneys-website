import type { BaseData, Language, WithId } from '$lib/types/common'

export type CategoryData = BaseData<{
    icon?: string
    name: string
}> & {
    seq: number
}

export type CreateCategoryRequest = {
    icon?: string
    name: string
    language: Language
    seq: number
}

export type UpdateCategoryRequest = WithId<CreateCategoryRequest>
