import type {
    APIError,
    APIResponse,
    ArticleData,
    CaseData,
    CreateCaseRequest,
    Language,
    UpdateCaseRequest,
} from '$lib/types'
import { ADMIN_URL, BASE_URL, TIMEOUT } from '$lib/constant'
import { getToken } from '$lib/utils'

async function save(
    req: CreateCaseRequest | UpdateCaseRequest,
): Promise<APIError | APIResponse<{ id: string }>> {
    try {
        const resp = await fetch(`${ADMIN_URL}/cases`, {
            method: 'id' in req ? 'PUT' : 'POST',
            headers: {
                'Content-Type': 'application/json',
                Authorization: getToken(),
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
            headers: {
                'Content-Type': 'application/json',
                Authorization: getToken(),
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
                    estimated_minutes: number
                    created_at: string
                }) => {
                    const date = new Date(e.created_at)

                    return {
                        id: e.id,
                        name: e.name,
                        estimatedMinutes: e.estimated_minutes,
                        createdAt: date,
                        createdAtString: `${date.getFullYear()}-${String(date.getMonth() + 1).padStart(2, '0')}-${String(date.getDate()).padStart(2, '0')}`,
                    }
                },
            )
        }

        return { error: false, cases: cases }
    } catch (error) {
        return { error: true, message: `Error: ${error}` }
    }
}

export const CaseServices = {
    save: save,
    list: list,
}
