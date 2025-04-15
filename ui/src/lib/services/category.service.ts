import type { CategoryData, CreateCategoryRequest, Language, UpdateCategoryRequest } from '$lib/types';
import { fromFetch } from 'rxjs/fetch';
import { ADMIN_URL, BASE_URL, TIMEOUT } from '$lib/constant';
import { getToken } from '$lib/utils';
import { map } from 'rxjs';

/**
 * The API endpoint of saving category
 * @param req
 */
function save(req: CreateCategoryRequest | UpdateCategoryRequest) {
	let method = 'id' in req ? 'PUT' : 'POST';

	return fromFetch(`${ADMIN_URL}/categories`, {
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
				return { error: true, message: `Error: ${resp.status}` };
			}
			return { error: false };
		})
	);
}

function list(language: Language) {
	return fromFetch(`${BASE_URL}/categories`, {
		method: 'GET',
		headers: {
			'Content-Type': 'application/json',
			'Accept-Language': language
		},
		signal: AbortSignal.timeout(TIMEOUT),
		selector: (resp) =>
			resp.json().then((value) => {
				return 'categories' in value
					? (value.categories as CategoryData[])
					: [];
			})
	});
}

function del(id: string) {
	return fromFetch(`${ADMIN_URL}/categories/${id}`, {
		method: 'DELETE',
		headers: {
			'Content-Type': 'application/json',
			Authorization: getToken()
		},
		signal: AbortSignal.timeout(TIMEOUT)
	}).pipe(
		map((resp) => {
			if (!resp.ok) {
				return { error: true, message: `Error: ${resp.status}` };
			}
			return { error: false };
		})
	);
}

/**
 * The API endpoint of retrieving the category by id and language
 * @param id the ID of service
 * @param language the language
 */
function retrieve(id: string, language: Language) {
	return fromFetch(`${BASE_URL}/categories/${id}`, {
		method: 'GET',
		headers: {
			'Content-Type': 'application/json',
			'Accept-Language': language
		},
		signal: AbortSignal.timeout(TIMEOUT),
		selector: (resp) =>
			resp.json().then((json) => {
				return 'category' in json
					? (json.category as CategoryData)
					: null;
			})
	});
}

export const CategoryService = {
	// save the content of category.
	save: save,
	// list all categories
	list: list,
	// delete the category
	delete: del,
	// retrieve the category
	retrieve: retrieve
};
