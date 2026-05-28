import type {
    APIError,
    APIResponse,
    CategoryData,
    CreateCategoryRequest,
    Language,
    UpdateCategoryRequest,
} from '$lib/types'
import { ADMIN_URL, BASE_URL, TIMEOUT } from '$lib/constant'
import { getToken } from '$lib/utils'

/**
 * Saves a category by sending either a creation or an update request to the server.
 * The HTTP method is determined based on whether the request contains an `id` property.
 *
 * @param {CreateCategoryRequest|UpdateCategoryRequest} req - The request object containing category data.
 * For a new category, provide a `CreateCategoryRequest`. For updating an existing category, provide an `UpdateCategoryRequest`.
 * @return {Promise<APIError | APIResponse<void>>} A promise that resolves to an object indicating the success or failure of the operation.
 * The `error` property is `true` if the save operation failed and `false` if it succeeded.
 * The `message` property is included in case of an error.
 */
async function save(
    req: CreateCategoryRequest | UpdateCategoryRequest
): Promise<APIError | APIResponse<void>> {
    try {
        const resp = await fetch(`${ADMIN_URL}/categories`, {
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
 * Fetches a list of categories from the server based on the provided language.
 *
 * @param {Language} language - The language to use for the request.
 * @return {Promise<APIError | APIResponse<{categories: CategoryData[]}>>}
 * A promise that resolves to an object containing either the list of categories or an error message.
 */
async function list(
    language: Language
): Promise<APIError | APIResponse<{ categories: CategoryData[] }>> {
    try {
        const resp = await fetch(`${BASE_URL}/categories`, {
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
        if ('categories' in json) {
            return {
                error: false,
                categories: json.categories as CategoryData[],
            }
        } else {
            return { error: true, message: `Error: can't decode json: ${json}` }
        }
    } catch (error) {
        return { error: true, message: `Error: ${error}` }
    }
}

/**
 * Deletes a category resource on the server based on the provided ID.
 *
 * @param {string} id The unique identifier of the category to be deleted.
 * @return {Promise<{error: boolean, message?: string}>} A promise that resolves to an object containing:
 *   - `error`: A boolean indicating whether there was an error.
 *   - `message`: A string containing an error message if an error occurred.
 */
async function del(id: string): Promise<APIError | APIResponse<void>> {
    try {
        const resp = await fetch(`${ADMIN_URL}/categories/${id}`, {
            method: 'DELETE',
            headers: {
                'Content-Type': 'application/json',
                Authorization: getToken(),
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

/**
 * Fetches category data by its identifier and language preference.
 *
 * @param {string} id - The identifier of the category to retrieve.
 * @param {Language} language - The language in which the data should be fetched.
 * @return {Promise<APIError | APIResponse<{category: CategoryData}>>} A promise that resolves with the category data or an error object.
 */
async function retrieve(
    id: string,
    language: Language
): Promise<APIError | APIResponse<{ category: CategoryData }>> {
    try {
        const resp = await fetch(`${BASE_URL}/categories/${id}`, {
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
        if ('category' in json) {
            return { error: false, category: json.category as CategoryData }
        } else {
            return { error: true, message: `Error: can't decode json: ${json}` }
        }
    } catch (error) {
        return { error: true, message: `Error: ${error}` }
    }
}

/**
 * CategoryService is an object that provides methods for managing categories.
 * It includes functionalities to save, list, delete, and retrieve category data.
 *
 * Methods:
 * - save: Function to save a new category or update an existing category.
 * - list: Function to retrieve a list of all categories.
 * - delete: Function to delete a specific category.
 * - retrieve: Function to retrieve details of a specific category by its identifier.
 */
export const CategoryService = {
    save: save,
    list: list,
    delete: del,
    retrieve: retrieve,
}
