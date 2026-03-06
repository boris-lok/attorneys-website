import type {
    APIError,
    APIResponse,
    CreateHomeRequest,
    HomeData,
    Language,
    UpdateHomeRequest,
} from '$lib/types'
import { ADMIN_URL, BASE_URL, TIMEOUT } from '$lib/constant'
import { getToken } from '$lib/utils'

/**
 * Sends a request to save a home entity. If the request contains an "id" property,
 * it performs an update (PUT request). Otherwise, it creates a new entity (POST request).
 *
 * @param {CreateHomeRequest|UpdateHomeRequest} req - The request object containing the home details to create or update.
 * @return {Promise<APIError | APIResponse<void>>} A promise that resolves to an object indicating if the operation was successful or not.
 */
async function save(
    req: CreateHomeRequest | UpdateHomeRequest,
): Promise<APIError | APIResponse<void>> {
    try {
        const resp = await fetch(`${ADMIN_URL}/home`, {
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

        return { error: false }
    } catch (error) {
        return { error: true, message: `Error: ${error}` }
    }
}

/**
 * Retrieves home data from the specified endpoint using the provided ID and language.
 *
 * @param {string} id - The unique identifier for the home resource to retrieve.
 * @param {Language} language - The language preference for the response.
 * @return {Promise<APIError | APIResponse<{home: HomeData}>>} A promise that resolves to an object containing the error status, home data if successful, or an error message if an issue occurs.
 */
async function retrieve(
    id: string,
    language: Language,
): Promise<APIError | APIResponse<{ home: HomeData }>> {
    try {
        const resp = await fetch(`${BASE_URL}/home/${id}`, {
            method: 'GET',
            headers: {
                'Content-Type': 'application/json',
                'Accept-Language': language,
            },
            signal: AbortSignal.timeout(TIMEOUT),
        })

        if (!resp.ok) {
            return { error: true, message: `Error: ${resp.status}` }
        }
        const json = await resp.json()
        if ('home' in json) {
            return { error: false, home: json.home as HomeData }
        } else {
            return { error: true, message: `Error: can't decode json: ${json}` }
        }
    } catch (error) {
        return { error: true, message: `Error: ${error}` }
    }
}

/**
 * Fetches data from the home endpoint using the specified language.
 *
 * @param {Language} language - The language preference for the request headers.
 * @return {Promise<APIError | APIResponse<{home: HomeData[]>}>} - A promise that resolves to an object containing either the fetched data or an error message.
 */
async function list(
    language: Language,
): Promise<APIError | APIResponse<{ home: HomeData[] }>> {
    try {
        const resp = await fetch(`${BASE_URL}/home`, {
            method: 'GET',
            headers: {
                'Content-Type': 'application/json',
                'Accept-Language': language,
            },
            signal: AbortSignal.timeout(TIMEOUT),
        })

        if (!resp.ok) {
            return { error: true, message: `Error: ${resp.status}` }
        }
        const json = await resp.json()
        if ('home' in json) {
            return { error: false, home: json.home as HomeData[] }
        } else {
            return { error: true, message: `Error: can't decode json: ${json}` }
        }
    } catch (error) {
        return { error: true, message: `Error: ${error}` }
    }
}

/**
 * HomeServices
 *
 * An object that provides a collection of methods for managing home-related services.
 *
 * Properties:
 * - save: Function that handles saving or storing data related to home services.
 * - retrieve: Function that retrieves specific data or details related to home services.
 * - list: Function that lists or provides an overview of all home services.
 *
 * Use this object to interact with and manage home service-related operations through its functions.
 */
export const HomeServices = {
    save: save,
    retrieve: retrieve,
    list: list,
}
