import type { CreateHomeRequest, UpdateHomeRequest } from '$lib/types'
import { fromFetch } from 'rxjs/fetch'
import { ADMIN_URL, TIMEOUT } from '$lib/constant'
import { getToken } from '$lib/utils'
import { map, of } from 'rxjs'

/**
 * The API endpoint of saving the content of home page
 * @param req
 */
function save(req: CreateHomeRequest | UpdateHomeRequest) {
    let method = 'POST'
    if ('id' in req) {
        method = 'PUT'
    }

    return fromFetch(`${ADMIN_URL}/home`, {
        method: method,
        headers: {
            'Content-Type': 'application/json',
            Authorization: getToken(),
        },
        body: JSON.stringify(req),
        signal: AbortSignal.timeout(TIMEOUT),
    }).pipe(
        map((resp) => {
            if (!resp.ok) {
                return { error: true, message: `Error: ${resp.status}` }
            }
            return { error: false }
        }),
    )
}

export const HomeServices = {
    // save the content of home page.
    save: save,
}
