import type { WithId } from '$lib/types/common'

export type CreateCaseRequest = {
    name: string
    estimatedMinutes: number
    billingCycle: number
    startedAt: Date
    endedAt: Date
}

export type UpdateCaseRequest = WithId<CreateCaseRequest>

export type CaseData = {
    id: string
    name: string
    usedMinutes: number
    estimatedMinutes: number
    createdAt: Date
    startedAt: Date
    endedAt: Date
    pendingLogs: number
    billingCycle: number
    settledAt: Date | null
}
