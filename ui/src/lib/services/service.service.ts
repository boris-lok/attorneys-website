import type {
    APIError,
    APIResponse,
    CreateServiceRequest,
    Language,
    ServiceData,
    UpdateServiceRequest,
} from '$lib/types'
import { ADMIN_URL, BASE_URL, TIMEOUT } from '$lib/constant'
import { getToken } from '$lib/utils'

/**
 * Sends a request to save a service. Determines whether to create or update based on the presence of an `id` in the request object.
 *
 * @param {CreateServiceRequest | UpdateServiceRequest} req The service request object. If an `id` property exists, it updates the service; otherwise, it creates a new service.
 * @return {Promise<APIError | APIResponse<void>>} A promise that resolves to an object indicating the success or failure of the save operation. If an error occurs, the object includes an error message.
 */
async function save(
    req: CreateServiceRequest | UpdateServiceRequest,
): Promise<APIError | APIResponse<void>> {
    try {
        const resp = await fetch(`${ADMIN_URL}/services`, {
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
 * Retrieves service data for a given service ID and language.
 *
 * @param {string} id - The unique identifier of the service to be retrieved.
 * @param {Language} language - The language in which the data should be retrieved.
 * @return {Promise<APIError | APIResponse<{service: ServiceData}>>}
 * A promise that resolves to an object containing an error flag and either the service data
 * or an error message.
 */
async function retrieve(
    id: string,
    language: Language,
): Promise<APIError | APIResponse<{ service: ServiceData }>> {
    try {
        const resp = await fetch(`${BASE_URL}/services/${id}`, {
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
        if ('service' in json) {
            return { error: false, service: json.service as ServiceData }
        } else {
            return { error: true, message: `Error: can't decode json: ${json}` }
        }
    } catch (error) {
        return { error: true, message: `Error: ${error}` }
    }
}

/**
 * Fetches a list of services from the server based on the provided language.
 *
 * @param {Language} language - The language code to retrieve services with the appropriate language settings.
 * @return {Promise<APIError | APIResponse<{services: ServiceData[]}>>}
 * Returns a promise that resolves with an object containing:
 * - `error` (boolean): Indicates whether the operation failed.
 * - `services` (optional array of ServiceData): The list of retrieved services if operation is successful.
 * - `message` (optional string): An error message if the operation fails.
 */
async function list(
    language: Language,
): Promise<APIError | APIResponse<{ services: ServiceData[] }>> {
    try {
        const resp = await fetch(`${BASE_URL}/services`, {
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
        if ('services' in json) {
            return { error: false, services: json.services as ServiceData[] }
        } else {
            return { error: true, message: `Error: can't decode json: ${json}` }
        }
    } catch (error) {
        return { error: true, message: `Error: ${error}` }
    }
}

/**
 * An object that provides methods for service operations such as saving, retrieving, and listing.
 *
 * @property {Function} save - A method used to save a service object.
 * @property {Function} retrieve - A method used to retrieve a specific service object.
 * @property {Function} list - A method used to list all available service objects.
 */
export const ServiceServices = {
    save: save,
    retrieve: retrieve,
    list: list,
}
