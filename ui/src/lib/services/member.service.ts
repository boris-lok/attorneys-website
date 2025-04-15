import type { CreateMemberRequest, Language, MemberData, SimpleMember, UpdateMemberRequest } from '$lib/types';
import { fromFetch } from 'rxjs/fetch';
import { ADMIN_URL, BASE_URL, TIMEOUT, UPLOAD_IMAGE_TIMEOUT } from '$lib/constant';
import { getToken } from '$lib/utils';
import { map, of, switchMap } from 'rxjs';

/**
 * The API endpoint of saving member
 * @param req
 */
function save(req: CreateMemberRequest | UpdateMemberRequest) {
	let method = 'id' in req ? 'PUT' : 'POST';

	return fromFetch(`${ADMIN_URL}/members`, {
		method: method,
		headers: {
			'Content-Type': 'application/json',
			Authorization: getToken()
		},
		body: JSON.stringify(req),
		signal: AbortSignal.timeout(TIMEOUT)
	}).pipe(
		switchMap((resp) => {
			if (!resp.ok) {
				return of({
					error: true,
					message: `Error: ${resp.status}`
				});
			}

			if ('id' in req) {
				return of({ error: false, id: req.id });
			} else {
				return resp.json().then((json) => {
					if ('id' in json) {
						return { error: false, id: json.id };
					}
					console.error('Missing id from response');
					return { error: false };
				});
			}
		})
	);
}

/**
 * The API endpoint of retrieving the member by id and language
 * @param id the ID of member
 * @param language the language
 */
function retrieve(id: string, language: Language) {
	return fromFetch(`${BASE_URL}/members/${id}`, {
		method: 'GET',
		headers: {
			'Content-Type': 'application/json',
			'Accept-Language': language
		},
		signal: AbortSignal.timeout(TIMEOUT),
		selector: (resp) =>
			resp.json().then((json) => {
				return 'member' in json ? (json.member as MemberData) : null;
			})
	});
}

/**
 * The API endpoint of listing all members by language
 * @param language the language
 */
function list(language: Language) {
	return fromFetch(`${BASE_URL}/members`, {
		method: 'GET',
		headers: {
			'Content-Type': 'application/json',
			'Accept-Language': language
		},
		signal: AbortSignal.timeout(TIMEOUT),
		selector: (resp) =>
			resp.json().then((value) => {
				return 'members' in value
					? (value.members as SimpleMember[])
					: [];
			})
	});
}

/**
 * The API endpoint of saving the member avatar
 * @param id The ID of member
 * @param file the avatar image file
 */
function saveAvatar(id: string, file: File) {
	const formData = new FormData();
	formData.append('avatar', file);

	return fromFetch(`${ADMIN_URL}/members/${id}/avatar`, {
		method: 'POST',
		body: formData,
		headers: {
			Authorization: getToken()
		},
		signal: AbortSignal.timeout(UPLOAD_IMAGE_TIMEOUT)
	}).pipe(
		map((resp) => {
			if (!resp.ok) {
				return { error: true, message: `Error: ${resp.status}` };
			}
			return { error: false };
		})
	);
}

export const MemberServices = {
	// save the content of member page.
	save: save,
	// retrieve the member
	retrieve: retrieve,
	// list all members
	list: list,
	// save member avatar
	saveAvatar: saveAvatar
};
