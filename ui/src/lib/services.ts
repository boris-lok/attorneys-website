import type { CreateMemberRequest, MemberData, SimpleMember, UpdateMemberRequest } from '$lib/models/Member';
import type { CreateHomeRequest, HomeData, UpdateHomeRequest } from '$lib/models/Home';
import type { Language } from '$lib/models/Language';
import { from, throwError } from 'rxjs';
import type { CreateServiceRequest, ServiceData, UpdateServiceRequest } from '$lib/models/Services';
import type { ArticleData, CreateArticleRequest, UpdateArticleRequest } from '$lib/models/Articles';
import type { ContactData, CreateContactRequest, UpdateContactRequest } from '$lib/models/ContactUs';
import type { LoginResponse } from '$lib/models/User';
import { user } from '../stores/userStore';

const BASE_URL = 'http://localhost:8081/api/v1';
const ADMIN_URL = `${BASE_URL}/admin`;
const TIMEOUT = 5000;

export const Members = {
	uploadAvatar: (id: string, file: File) => {
		const formData = new FormData();
		formData.append('avatar', file);

		const request: Promise<Response> = fetch(
			`${ADMIN_URL}/members/${id}/avatar`,
			{
				method: 'POST',
				body: formData
			}
		);

		return from(request);
	},
	list: (language: Language) => {
		const request = fetch(
			`${BASE_URL}/members`,
			{
				method: 'GET',
				headers: {
					'Content-Type': 'application/json',
					'Accept-Language': language
				},
				signal: AbortSignal.timeout(TIMEOUT)
			}
		).then((res) => res.json())
			.then((json: Object) => {
				const data = 'members' in json ? json.members as SimpleMember[] : [];
				return data;
			});

		return from(request);
	},
	save: (req: CreateMemberRequest | UpdateMemberRequest) => {
		let method = 'POST';
		if ('id' in req) {
			method = 'PUT';
		}

		const request = fetch(
			`${ADMIN_URL}/members`,
			{
				method: method,
				headers: {
					'Content-Type': 'application/json'
				},
				body: JSON.stringify(req),
				signal: AbortSignal.timeout(TIMEOUT)
			}
		).then(res => {
			if (method === 'POST') {
				let id = res.json()
					.then(json => {
						if ('id' in json) {
							return json.id as string;
						}

						return null;
					});
				return id;
			}
		})
			.then(e => {
				if (method === 'PUT' && 'id' in req) {
					return req.id;
				} else if (e) {
					return e;
				} else {
					return null;
				}
			});

		return from(request);
	},
	retrieve: (id: string, language: Language) => {
		const request = fetch(`${BASE_URL}/members/${id}`, {
			method: 'GET',
			headers: {
				'Content-Type': 'application/json',
				'Accept-Language': language
			},
			signal: AbortSignal.timeout(TIMEOUT)
		})
			.then(res => res.json())
			.then(res => 'member' in res ? res.member as MemberData : null);

		return from(request);
	}
};

export const Home = {
	list: (language: Language) => {
		const request = fetch(
			`${BASE_URL}/home`,
			{
				method: 'GET',
				headers: {
					'Content-Type': 'application/json',
					'Accept-Language': language
				},
				signal: AbortSignal.timeout(TIMEOUT)
			}
		).then((res) => res.json())
			.then((json: Object) => {
				const data = 'home' in json ? json.home as HomeData[] : [];
				if (data.length === 0) {
					return null;
				}
				return data[0];
			});

		return from(request);
	},
	retrieve: (id: string, language: Language) => {
		const request = fetch(`${BASE_URL}/home/${id}`, {
			method: 'GET',
			headers: {
				'Content-Type': 'application/json',
				'Accept-Language': language
			},
			signal: AbortSignal.timeout(TIMEOUT)
		})
			.then(res => res.json())
			.then(res => 'home' in res ? res.home as HomeData : null);

		return from(request);
	},
	save: (req: CreateHomeRequest | UpdateHomeRequest) => {
		let method = 'POST';
		if ('id' in req) {
			method = 'PUT';
		}

		const request = fetch(
			`${ADMIN_URL}/home`,
			{
				method: method,
				headers: {
					'Content-Type': 'application/json'
				},
				body: JSON.stringify(req),
				signal: AbortSignal.timeout(TIMEOUT)
			}
		);

		return from(request);
	}
};

export const Services = {
	list: (language: Language) => {
		const request = fetch(
			`${BASE_URL}/services`,
			{
				method: 'GET',
				headers: {
					'Content-Type': 'application/json',
					'Accept-Language': language
				},
				signal: AbortSignal.timeout(TIMEOUT)
			}
		).then((res) => res.json())
			.then((json: Object) => {
				const data = 'services' in json ? json.services as ServiceData[] : [];
				return data;
			});

		return from(request);
	},
	retrieve: (id: string, language: Language) => {
		const request = fetch(`${BASE_URL}/services/${id}`, {
			method: 'GET',
			headers: {
				'Content-Type': 'application/json',
				'Accept-Language': language
			},
			signal: AbortSignal.timeout(TIMEOUT)
		})
			.then(res => res.json())
			.then(res => 'service' in res ? res.service as ServiceData : null);

		return from(request);
	},
	save: (req: CreateServiceRequest | UpdateServiceRequest) => {
		let method = 'POST';
		if ('id' in req) {
			method = 'PUT';
		}

		const request = fetch(
			`${ADMIN_URL}/services`,
			{
				method: method,
				headers: {
					'Content-Type': 'application/json'
				},
				body: JSON.stringify(req),
				signal: AbortSignal.timeout(TIMEOUT)
			}
		);

		return from(request);
	}
};

export const Articles = {
	list: (language: Language) => {
		const request = fetch(
			`${BASE_URL}/articles`,
			{
				method: 'GET',
				headers: {
					'Content-Type': 'application/json',
					'Accept-Language': language
				},
				signal: AbortSignal.timeout(TIMEOUT)
			}
		).then((res) => res.json())
			.then((json: Object) => {
				const data = 'articles' in json ? json.articles as ArticleData[] : [];
				return data;
			});

		return from(request);
	},
	retrieve: (id: string, language: Language) => {
		const request = fetch(`${BASE_URL}/articles/${id}`, {
			method: 'GET',
			headers: {
				'Content-Type': 'application/json',
				'Accept-Language': language
			},
			signal: AbortSignal.timeout(TIMEOUT)
		})
			.then(res => res.json())
			.then(res => 'article' in res ? res.article as ArticleData : null);

		return from(request);
	},
	save: (req: CreateArticleRequest | UpdateArticleRequest) => {
		let method = 'POST';
		if ('id' in req) {
			method = 'PUT';
		}

		const request = fetch(
			`${ADMIN_URL}/articles`,
			{
				method: method,
				headers: {
					'Content-Type': 'application/json'
				},
				body: JSON.stringify(req),
				signal: AbortSignal.timeout(TIMEOUT)
			}
		);

		return from(request);
	}
};

export const Contacts = {
	list: (language: Language) => {
		const request = fetch(
			`${BASE_URL}/contact`,
			{
				method: 'GET',
				headers: {
					'Content-Type': 'application/json',
					'Accept-Language': language
				},
				signal: AbortSignal.timeout(TIMEOUT)
			}
		).then((res) => res.json())
			.then((json: Object) => {
				const data = 'contact' in json ? json.contact as ContactData[] : [];
				if (data.length === 0) {
					return null;
				}
				return data[0];
			});

		return from(request);
	},
	retrieve: (id: string, language: Language) => {
		const request = fetch(`${BASE_URL}/contact/${id}`, {
			method: 'GET',
			headers: {
				'Content-Type': 'application/json',
				'Accept-Language': language
			},
			signal: AbortSignal.timeout(TIMEOUT)
		})
			.then(res => res.json())
			.then(res => 'contact' in res ? res.contact as ContactData : null);

		return from(request);
	},
	save: (req: CreateContactRequest | UpdateContactRequest) => {
		let method = 'POST';
		if ('id' in req) {
			method = 'PUT';
		}

		const request = fetch(
			`${ADMIN_URL}/contact`,
			{
				method: method,
				headers: {
					'Content-Type': 'application/json'
				},
				body: JSON.stringify(req),
				signal: AbortSignal.timeout(TIMEOUT)
			}
		);

		return from(request);
	}
};

export const Users = {
	login: (username: string, password: string) => {
		const request = fetch(`${ADMIN_URL}/login`, {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify({
				username: username,
				password: password
			}),
			signal: AbortSignal.timeout(TIMEOUT)
		})
			.then(res => res.json())
			.then(res => res as LoginResponse);

		return from(request);
	},
	logout: () => {
		const u = user.get();
		if (!u) {
			throw throwError(() => 'User not logged in');
		}

		const request = fetch(`${ADMIN_URL}/logout`, {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json',
				Authorization: `Bearer ${u.token}`
			},
			signal: AbortSignal.timeout(TIMEOUT)
		});

		return from(request);
	}
};