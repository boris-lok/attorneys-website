import type { BaseData, Language, WithId } from '$lib/types/common'

export type MemberData = BaseData<{
    name: string
    description: string
}> & {
    avatar?: string
    seq: number
}

export type SimpleMember = {
    id: string
    name: string
    seq: number
    avatar?: string
}

export type ImageData = {
    lgImage: string
    smImage: string
}

export type CreateMemberRequest = {
    name: string
    description: string
    language: Language
    seq: number
}

export type UpdateMemberRequest = WithId<CreateMemberRequest>
