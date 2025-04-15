import type { CreateHomeRequest, HomeData, Language, UpdateHomeRequest } from '$lib/types'
import { fromFetch } from 'rxjs/fetch'
import { ADMIN_URL, BASE_URL, TIMEOUT } from '$lib/constant'
import { getToken } from '$lib/utils'
import { map } from 'rxjs'

/**
 * The API endpoint of saving the content of home page
 * @param req
 */
function save(req: CreateHomeRequest | UpdateHomeRequest) {
    let method = 'id' in req ? 'PUT' : 'POST'

    return fromFetch(`${ADMIN_URL}/home`, {
        method: method,
        headers: {
            'Content-Type': 'application/json',
            Authorization: getToken()
        },
        body: JSON.stringify(req),
        signal: AbortSignal.timeout(TIMEOUT)
    }).pipe(
        map((resp) => {
            if (!resp.ok) {
                return { error: true, message: `Error: ${resp.status}` }
            }
            return { error: false }
        })
    )
}

/**
 * The API endpoint of retrieving the content of home page.
 * @param id The id of the Home data
 * @param language The language of the data
 */
function retrieve(id: string, language: Language) {
    return fromFetch(`${BASE_URL}/home/${id}`, {
        method: 'GET',
        headers: {
            'Content-Type': 'application/json',
            'Accept-Language': language
        },
        signal: AbortSignal.timeout(TIMEOUT),
        selector: (resp) =>
            resp.json().then((json) => {
                return 'home' in json ? (json.home as HomeData) : null
            })
    })
}

/**
 * The API endpoint of list all home
 * @param language the language of the data
 */
function list(language: Language) {
    return fromFetch(`${BASE_URL}/home`, {
        method: 'GET',
        headers: {
            'Content-Type': 'application/json',
            'Accept-Language': language
        },
        signal: AbortSignal.timeout(TIMEOUT),
        selector: (resp) =>
            resp.json().then((value) => {
                return 'home' in value ? (value.home as HomeData[]) : []
            })
    })
}

export const HomeServices = {
    // save the content of home page.
    save: save,
    // retrieve the home
    retrieve: retrieve,
    // list all home
    list: list
}
