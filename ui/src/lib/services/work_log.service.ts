// The request of creating work log
import type { APIError, APIResponse, CaseData } from '$lib/types'
import { ADMIN_URL, TIMEOUT } from '$lib/constant'
import { getToken } from '$lib/utils'

export type CreateWorkLogRequest = {
    caseId: string
    startedAt: Date
    duration: number
    description: string
    collaboratorIds: string[]
}

export type WorkLog = {
    id: string
    startedAt: Date
    duration: number
    description: string
    isCollaborative: boolean
    collaborators: Collaborator[]
    user: SimpleUser
    status: string
}

export type SimpleUser = {
    id: string
    name: string
}

export type Collaborator = {
    parentId: string
    userId: string
    name: string
    status: string
}

// The request of updating work log
export type UpdateWorkLogRequest = CreateWorkLogRequest & { id: string }

async function save(
    req: CreateWorkLogRequest | UpdateWorkLogRequest,
): Promise<APIError | APIResponse<{ id: string }>> {
    try {
        const resp = await fetch(`${ADMIN_URL}/work_logs`, {
            method: 'id' in req ? 'PUT' : 'POST',
            headers: {
                'Content-Type': 'application/json',
                Authorization: getToken(),
            },
            body: JSON.stringify({
                ...('id' in req ? { id: req.id } : {}),
                case_id: req.caseId,
                started_at: req.startedAt,
                duration: req.duration,
                description: req.description,
                collaborator_ids: req.collaboratorIds,
            }),
            signal: AbortSignal.timeout(TIMEOUT),
        })

        if (!resp.ok) {
            return { error: true, message: `Error: ${resp.status}` }
        }

        if ('id' in req) {
            return { error: false, id: req.id }
        }

        const json = await resp.json()
        if ('id' in json) {
            return { error: false, id: json.id }
        }

        return { error: true, message: 'Missing id from response' }
    } catch (error) {
        return { error: true, message: `Error: ${error}` }
    }
}

async function list(
    caseId: string,
): Promise<APIError | APIResponse<{ logs: WorkLog[] }>> {
    let url = `${ADMIN_URL}/work_logs?case_id=${caseId}`

    try {
        const resp = await fetch(url, {
            method: 'GET',
            headers: {
                'Content-Type': 'application/json',
                Authorization: getToken(),
            },
            signal: AbortSignal.timeout(TIMEOUT),
        })

        const json = await resp.json()
        let workLogs = []

        if ('work_logs' in json && json.work_logs.length > 0) {
            workLogs = json.work_logs.map(
                (e: {
                    id: string
                    started_at: string
                    duration: number
                    description: string
                    is_collaborative: boolean
                    collaborators: {
                        parent_id: string
                        user_id: string
                        name: string
                        status: string
                    }[]
                    user: { user_id: string; name: string }
                    status: string
                }) => {
                    return {
                        id: e.id,
                        startedAt: new Date(e.started_at),
                        duration: e.duration,
                        description: e.description,
                        isCollaborative: e.is_collaborative,
                        collaborators: e.collaborators.map((collaborator) => ({
                            parentId: collaborator.parent_id,
                            userId: collaborator.user_id,
                            name: collaborator.name,
                            status: collaborator.status,
                        })),
                        user: {
                            id: e.user.user_id,
                            name: e.user.name,
                        },
                        status: e.status,
                    } as WorkLog
                },
            )
        }

        return { error: false, logs: workLogs }
    } catch (error) {
        return { error: true, message: `Error: ${error}` }
    }
}
export const WorkLogServices = {
    save: save,
    list: list,
}
