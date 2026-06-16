import type { BaseData, Language, WithId } from '$lib/types/common'

export type HomeData = BaseData<{ data: string }>

export type CreateHomeRequest = {
    data: string
    language: Language
    seq: number
}

export type UpdateHomeRequest = WithId<CreateHomeRequest>
