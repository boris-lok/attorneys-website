import type { BaseData, Language, WithId } from '$lib/types/common'

export type ServiceData = BaseData<{
    title: string
    data: string
    icon?: string
}>

export type CreateServiceRequest = {
    title: string
    data: string
    language: Language
    seq: number
}

export type UpdateServiceRequest = WithId<CreateServiceRequest>
