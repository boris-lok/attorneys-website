import type { WithId } from '$lib/types/common'

export type CreateCaseRequest = {
    name: string
    estimated_minutes: number
    billing_cycle: number
    started_at: Date
    ended_at: Date
}

export type UpdateCaseRequest = WithId<Partial<CreateCaseRequest>> & {
    closed?: boolean
}

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
    closed: boolean
}
