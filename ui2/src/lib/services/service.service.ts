import type { CreateServiceRequest, Language, ServiceData, UpdateServiceRequest } from '$lib/types'
import { fromFetch } from 'rxjs/fetch'
import { ADMIN_URL, BASE_URL, TIMEOUT } from '$lib/constant'
import { getToken } from '$lib/utils'
import { map } from 'rxjs'

/**
 * The API endpoint of saving service
 * @param req
 */
function save(req: CreateServiceRequest | UpdateServiceRequest) {
    let method = 'id' in req ? 'PUT' : 'POST'

    return fromFetch(`${ADMIN_URL}/services`, {
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

/**
 * The API endpoint of retrieving the service by id and language
 * @param id the ID of service
 * @param language the language
 */
function retrieve(id: string, language: Language) {
    return fromFetch(`${BASE_URL}/services/${id}`, {
        method: 'GET',
        headers: {
            'Content-Type': 'application/json',
            'Accept-Language': language,
        },
        signal: AbortSignal.timeout(TIMEOUT),
        selector: (resp) =>
            resp.json().then((json) => {
                return 'service' in json ? (json.service as ServiceData) : null
            }),
    })
}

function list(language: Language) {
    return fromFetch(`${BASE_URL}/services`, {
        method: 'GET',
        headers: {
            'Content-Type': 'application/json',
            'Accept-Language': language,
        },
        signal: AbortSignal.timeout(TIMEOUT),
        selector: (resp) =>
            resp.json().then((value) => {
                return 'services' in value
                    ? (value.services as ServiceData[])
                    : []
            }),
    })
}

export const ServiceServices = {
    // save the content of service page.
    save: save,
    // retrieve the service
    retrieve: retrieve,
    // list all services
    list: list,
}
