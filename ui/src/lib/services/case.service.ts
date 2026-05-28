import type {
    APIError,
    APIResponse,
    CaseData,
    CreateCaseRequest,
    UpdateCaseRequest,
} from '$lib/types'
import { ADMIN_URL, TIMEOUT } from '$lib/constant'

async function save(
    req: CreateCaseRequest | UpdateCaseRequest
): Promise<APIError | APIResponse<{ id: string }>> {
    try {
        const resp = await fetch(`${ADMIN_URL}/cases`, {
            method: 'id' in req ? 'PUT' : 'POST',
            credentials: 'include',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify(req),
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

async function list(): Promise<APIError | APIResponse<{ cases: CaseData[] }>> {
    let url = `${ADMIN_URL}/cases`

    try {
        const resp = await fetch(url, {
            method: 'GET',
            credentials: 'include',
            headers: {
                'Content-Type': 'application/json',
            },
            signal: AbortSignal.timeout(TIMEOUT),
        })

        const json = await resp.json()
        let cases = []

        if ('cases' in json && json.cases.length > 0) {
            cases = json.cases.map(
                (e: {
                    id: string
                    name: string
                    used_minutes: number
                    estimated_minutes: number
                    created_at: string
                    started_at: string
                    ended_at: string
                    pending_logs: number
                }) => {
                    return {
                        id: e.id,
                        name: e.name,
                        usedMinutes: e.used_minutes,
                        estimatedMinutes: e.estimated_minutes,
                        createdAt: new Date(e.created_at),
                        startedAt: new Date(e.started_at),
                        endedAt: new Date(e.ended_at),
                        pendingLogs: e.pending_logs,
                    }
                }
            )
        }

        return { error: false, cases: cases }
    } catch (error) {
        return { error: true, message: `Error: ${error}` }
    }
}

async function del(id: string): Promise<APIError | APIResponse<void>> {
    try {
        const resp = await fetch(`${ADMIN_URL}/cases/${id}`, {
            method: 'DELETE',
            credentials: 'include',
            headers: {
                'Content-Type': 'application/json',
            },
            signal: AbortSignal.timeout(TIMEOUT),
        })
        if (!resp.ok) {
            return { error: true, message: `Error: ${resp.status}` }
        }
        return { error: false }
    } catch (error) {
        return { error: true, message: `Error: ${error}` }
    }
}

export const CaseServices = {
    save: save,
    list: list,
    delete: del,
}
