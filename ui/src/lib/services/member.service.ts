import type { CreateMemberRequest, Language, MemberData, SimpleMember, UpdateMemberRequest } from '$lib/types'
import { ADMIN_URL, BASE_URL, TIMEOUT, UPLOAD_IMAGE_TIMEOUT } from '$lib/constant'
import { getToken } from '$lib/utils'

/**
 * Saves a new member or updates an existing member based on the request object provided.
 *
 * @param {CreateMemberRequest | UpdateMemberRequest} req The request object containing the data for creating or updating a member.
 *        Pass `CreateMemberRequest` to create a new member or `UpdateMemberRequest` to update an existing member.
 * @return {Promise<{ error: boolean, message?: string, id?: string }>} A promise that resolves with an object indicating the success or failure of the operation.
 *         On success, the object contains `error: false` and the `id` of the member.
 *         On failure, the object contains `error: true` and an error `message`.
 */
async function save(
    req: CreateMemberRequest | UpdateMemberRequest,
): Promise<{ error: boolean; message?: string; id?: string }> {
    try {
        const resp = await fetch(`${ADMIN_URL}/members`, {
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

        const json = await resp.json()
        if ('id' in json) {
            return { error: false, id: json.id }
        }
        return { error: true, message: 'Missing id from response' }
    } catch (error) {
        return { error: true, message: `Error: ${error}` }
    }
}

/**
 * Fetches member data by a given ID from a remote service.
 * The request is performed using the specified language header for localization.
 *
 * @param {string} id - The unique identifier of the member to be retrieved.
 * @param {Language} language - The preferred language to be included in the request headers.
 * @return {Promise<{error: boolean, message?: string, member?: MemberData}>} - A promise that resolves to an object indicating success or failure.
 * If successful, the object contains the member data. If unsuccessful, an error flag and message are provided.
 */
async function retrieve(
    id: string,
    language: Language,
): Promise<{ error: boolean; message?: string; member?: MemberData }> {
    try {
        const resp = await fetch(`${BASE_URL}/members/${id}`, {
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
        if ('member' in json) {
            return { error: false, member: json.member as MemberData }
        } else {
            return { error: true, message: `Error: can't decode json: ${json}` }
        }
    } catch (error) {
        return { error: true, message: `Error: ${error}` }
    }
}

/**
 * Fetches a list of members from a remote server.
 *
 * @param {Language} language - The preferred language for the response, used to set the Accept-Language header.
 * @return {Promise<{ error: boolean, message?: string, members?: SimpleMember[] }>} A promise that resolves to an object containing a list of members or an error message.
 */
async function list(
    language: Language,
): Promise<{ error: boolean; message?: string; members?: SimpleMember[] }> {
    try {
        const resp = await fetch(`${BASE_URL}/members`, {
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
        return {
            error: false,
            members: 'members' in json ? (json.members as SimpleMember[]) : [],
        }
    } catch (error) {
        return { error: true, message: `Error: ${error}` }
    }
}

/**
 * Updates the avatar for a specified member by uploading the provided file.
 *
 * @param {string} id - The unique identifier of the member whose avatar is being updated.
 * @param {File} file - The avatar image file to upload.
 * @return {Promise<{error: boolean, message?: string}>} A promise that resolves to an object indicating the success or failure of the upload. If the upload fails, the object contains an error message.
 */
async function saveAvatar(
    id: string,
    file: File,
): Promise<{ error: boolean; message?: string }> {
    try {
        const formData = new FormData()
        formData.append('avatar', file)
        const resp = await fetch(`${ADMIN_URL}/members/${id}/avatar`, {
            method: 'POST',
            body: formData,
            headers: {
                Authorization: getToken(),
            },
            signal: AbortSignal.timeout(UPLOAD_IMAGE_TIMEOUT),
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
 * MemberServices is an object that provides various operations related to member management,
 * including saving, retrieving, listing members, and managing member avatars.
 *
 * Properties:
 * - save: A function that handles saving member data.
 * - retrieve: A function that retrieves member data based on specific criteria.
 * - list: A function that lists all members or members matching certain filters.
 * - saveAvatar: A function that handles saving member avatars.
 */
export const MemberServices = {
    save: save,
    retrieve: retrieve,
    list: list,
    saveAvatar: saveAvatar,
}
