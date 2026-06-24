import type { WithId } from '$lib/types/common'

export type CreateWorkLogRequest = {
    caseId: string
    startedAt: Date
    duration: number
    description: string
    collaboratorIds: string[]
}

export type UpdateWorkLogRequest = WithId<CreateWorkLogRequest>

export type WorkLog = {
    id: string
    startedAt: Date
    endedAt: Date
    duration: number
    description: string
    isCollaborative: boolean
    collaborators: Collaborator[]
    user: Creator
}

export type Creator = {
    id: string
    name: string
}

export type Collaborator = {
    parentId: string
    userId: string
    name: string
    status: string
}

export type PendingWorkLog = {
    id: string
    startedAt: Date
    endedAt: Date
    duration: number
    description: string
    user: {
        id: string
        name: string
    }
}

export type SimpleUser = {
    id: string
    nickname: string
    roles: string[]
}
