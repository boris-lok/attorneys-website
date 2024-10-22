import type { CreateMemberRequest, MemberData, SimpleMember, UpdateMemberRequest } from '$lib/models/Member';
import type { CreateHomeRequest, HomeData, UpdateHomeRequest } from '$lib/models/Home';
import type { Language } from '$lib/models/Language';
import { from, throwError } from 'rxjs';
import type { CreateServiceRequest, ServiceData, UpdateServiceRequest } from '$lib/models/Services';
import type { ArticleData, CreateArticleRequest, UpdateArticleRequest } from '$lib/models/Articles';
import type { ContactData, CreateContactRequest, UpdateContactRequest } from '$lib/models/ContactUs';
import { user } from '../stores/userStore';
import type { LoginResponse } from '$lib/models/User';

const BASE_URL = `${import.meta.env.VITE_BACKEND_HOST}:${import.meta.env.VITE_BACKEND_PORT}/api/v1`;
const ADMIN_URL = `${BASE_URL}/admin`;
const TIMEOUT = 5000;

function getToken() {
	const u = user.get();
	if (!u) {
		throw throwError(() => 'User not logged in');
	}

	return `Bearer ${u.token}`;
}

async function handleError(res: Response, defaultMessage: string) {
	if (!res.ok) {
		let json = await res.json();
		console.error(json);
		if ('message' in json && typeof json.message === 'string') {
			throw new Error(json.message);
		} else {
			throw new Error(defaultMessage);
		}
	}
}

export const Members = {
	uploadAvatar: (id: string, file: File) => {
		async function __inner() {
			const formData = new FormData();
			formData.append('avatar', file);

			const response = await fetch(
				`${ADMIN_URL}/members/${id}/avatar`,
				{
					method: 'POST',
					body: formData,
					headers: {
						Authorization: getToken()
					}
				}
			);

			await handleError(response, 'failed to upload avatar');
		}

		return from(__inner());
	},
	list: (language: Language) => {
		async function __inner() {
			const response = await fetch(
				`${BASE_URL}/members`,
				{
					method: 'GET',
					headers: {
						'Content-Type': 'application/json',
						'Accept-Language': language
					},
					signal: AbortSignal.timeout(TIMEOUT)
				}
			);

			await handleError(response, 'failed to list members');

			const json = await response.json();
			return 'members' in json ? json.members as SimpleMember[] : [];
		}

		return from(__inner());
	},
	save: (req: CreateMemberRequest | UpdateMemberRequest) => {
		let method = 'POST';
		if ('id' in req) {
			method = 'PUT';
		}

		async function __inner() {
			const response = await fetch(
				`${ADMIN_URL}/members`,
				{
					method: method,
					headers: {
						'Content-Type': 'application/json',
						Authorization: getToken()
					},
					body: JSON.stringify(req),
					signal: AbortSignal.timeout(TIMEOUT)
				}
			);

			await handleError(response, 'failed to save member');

			if (method === 'POST') {
				const json = await response.json();
				return 'id' in json ? json.id : null;
			} else if (method === 'PUT' && 'id' in req) {
				return req.id;
			}

			return null;
		}

		return from(__inner());
	},
	retrieve: (id: string, language: Language) => {
		async function __inner() {
			const response = await fetch(`${BASE_URL}/members/${id}`, {
				method: 'GET',
				headers: {
					'Content-Type': 'application/json',
					'Accept-Language': language
				},
				signal: AbortSignal.timeout(TIMEOUT)
			});

			await handleError(response, 'failed to retrieve member');
			const json = await response.json();

			return 'member' in json ? json.member as MemberData : null;
		}

		return from(__inner());
	}
};

export const Home = {
	list: (language: Language) => {
		async function __inner() {
			const response = await fetch(
				`${BASE_URL}/home`,
				{
					method: 'GET',
					headers: {
						'Content-Type': 'application/json',
						'Accept-Language': language
					},
					signal: AbortSignal.timeout(TIMEOUT)
				}
			);

			await handleError(response, 'failed to list home');

			const json = await response.json();
			const data = 'home' in json ? json.home as HomeData[] : [];
			return data.length === 0 ? null : data[0];
		}

		return from(__inner());
	},
	retrieve: (id: string, language: Language) => {
		async function __inner() {
			const response = await fetch(`${BASE_URL}/home/${id}`, {
				method: 'GET',
				headers: {
					'Content-Type': 'application/json',
					'Accept-Language': language
				},
				signal: AbortSignal.timeout(TIMEOUT)
			});

			await handleError(response, 'failed to retrieve home');
			const json = await response.json();

			return 'home' in json ? json.home as HomeData : null;
		}

		return from(__inner());
	},
	save: (req: CreateHomeRequest | UpdateHomeRequest) => {
		let method = 'POST';
		if ('id' in req) {
			method = 'PUT';
		}

		async function __inner() {
			const response = await fetch(
				`${ADMIN_URL}/home`,
				{
					method: method,
					headers: {
						'Content-Type': 'application/json',
						Authorization: getToken()
					},
					body: JSON.stringify(req),
					signal: AbortSignal.timeout(TIMEOUT)
				}
			);

			await handleError(response, 'failed to save home');
		}

		return from(__inner());
	}
};

export const Services = {
	list: (language: Language) => {
		async function __inner() {
			const response = await fetch(
				`${BASE_URL}/services`,
				{
					method: 'GET',
					headers: {
						'Content-Type': 'application/json',
						'Accept-Language': language
					},
					signal: AbortSignal.timeout(TIMEOUT)
				}
			);

			await handleError(response, 'failed to list services');
			const json = await response.json();

			return 'services' in json ? json.services as ServiceData[] : [];
		}

		return from(__inner());
	},
	retrieve: (id: string, language: Language) => {
		async function __inner() {
			const response = await fetch(`${BASE_URL}/services/${id}`, {
				method: 'GET',
				headers: {
					'Content-Type': 'application/json',
					'Accept-Language': language
				},
				signal: AbortSignal.timeout(TIMEOUT)
			});

			await handleError(response, 'failed to retrieve service');
			const json = await response.json();

			return 'service' in json ? json.service as ServiceData : null;
		}

		return from(__inner());
	},
	save: (req: CreateServiceRequest | UpdateServiceRequest) => {
		let method = 'POST';
		if ('id' in req) {
			method = 'PUT';
		}

		async function __inner() {
			const response = await fetch(
				`${ADMIN_URL}/services`,
				{
					method: method,
					headers: {
						'Content-Type': 'application/json',
						Authorization: getToken()
					},
					body: JSON.stringify(req),
					signal: AbortSignal.timeout(TIMEOUT)
				}
			);
			await handleError(response, 'failed to save service');
		}

		return from(__inner());
	}
};

export const Articles = {
	list: (language: Language, page: number, pageSize: number) => {
		async function __inner() {
			const response = await fetch(
				`${BASE_URL}/articles?page=${page}&page_size=${pageSize}`,
				{
					method: 'GET',
					headers: {
						'Content-Type': 'application/json',
						'Accept-Language': language
					},
					signal: AbortSignal.timeout(TIMEOUT)
				}
			);

			await handleError(response, 'failed to list articles');

			const json = await response.json();
			const articles = 'articles' in json ? json.articles as ArticleData[] : [];
			return {
				articles: articles,
				total: json.total
			};
		}

		return from(__inner());
	},
	retrieve: (id: string, language: Language) => {
		async function __inner() {
			const response = await fetch(`${BASE_URL}/articles/${id}`, {
				method: 'GET',
				headers: {
					'Content-Type': 'application/json',
					'Accept-Language': language
				},
				signal: AbortSignal.timeout(TIMEOUT)
			});

			await handleError(response, 'failed to retrieve article');

			const json = await response.json();
			return 'article' in json ? json.article as ArticleData : null;
		}

		return from(__inner());
	},
	save: (req: CreateArticleRequest | UpdateArticleRequest) => {
		let method = 'POST';
		if ('id' in req) {
			method = 'PUT';
		}

		async function __inner() {
			const response = await fetch(
				`${ADMIN_URL}/articles`,
				{
					method: method,
					headers: {
						'Content-Type': 'application/json',
						Authorization: getToken()
					},
					body: JSON.stringify(req),
					signal: AbortSignal.timeout(TIMEOUT)
				}
			);

			await handleError(response, 'failed to save article');
		}

		return from(__inner());
	}
};

export const Contacts = {
	list: (language: Language) => {
		async function __inner() {
			const response = await fetch(
				`${BASE_URL}/contact`,
				{
					method: 'GET',
					headers: {
						'Content-Type': 'application/json',
						'Accept-Language': language
					},
					signal: AbortSignal.timeout(TIMEOUT)
				}
			);
			await handleError(response, 'failed to list contact');

			let json = await response.json();

			const data = 'contact' in json ? json.contact as ContactData[] : [];
			return data.length === 0 ? null : data[0];
		}

		return from(__inner());
	},
	retrieve: (id: string, language: Language) => {
		async function __inner() {
			const response = await fetch(`${BASE_URL}/contact/${id}`, {
				method: 'GET',
				headers: {
					'Content-Type': 'application/json',
					'Accept-Language': language
				},
				signal: AbortSignal.timeout(TIMEOUT)
			});

			await handleError(response, 'failed to retrieve contact');

			let json = await response.json();

			if ('contact' in json) {
				return json.contact as ContactData;
			}
			return null;
		}

		return from(__inner());
	},
	save: (req: CreateContactRequest | UpdateContactRequest) => {
		let method = 'POST';
		if ('id' in req) {
			method = 'PUT';
		}

		async function __inner() {
			const response = await fetch(
				`${ADMIN_URL}/contact`,
				{
					method: method,
					headers: {
						'Content-Type': 'application/json',
						Authorization: getToken()
					},
					body: JSON.stringify(req),
					signal: AbortSignal.timeout(TIMEOUT)
				}
			);

			await handleError(response, 'failed to save contact');
		}

		return from(__inner());
	}
};

export const Users = {
	login: (username: string, password: string) => {
		async function __inner() {
			const response = await fetch(`${ADMIN_URL}/login`, {
				method: 'POST',
				headers: {
					'Content-Type': 'application/json'
				},
				body: JSON.stringify({
					username: username,
					password: password
				}),
				signal: AbortSignal.timeout(TIMEOUT)
			});

			await handleError(response, 'failed to login');

			let json = await response.json();

			if ('token' in json && 'user_id' in json && 'username' in json) {
				return {
					userId: json.user_id,
					username: json.username,
					token: json.token
				} as LoginResponse;
			}

			return null;
		}

		return from(__inner());
	},
	logout: () => {
		async function __inner() {
			const response = await fetch(`${ADMIN_URL}/logout`, {
				method: 'POST',
				headers: {
					'Content-Type': 'application/json',
					Authorization: getToken()
				},
				signal: AbortSignal.timeout(TIMEOUT)
			});

			await handleError(response, 'failed to logout');
		}

		return from(__inner());
	}
};