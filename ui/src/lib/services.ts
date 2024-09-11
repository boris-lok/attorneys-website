import type {Member} from "$lib/models/Member";
import type {Language} from "$lib/models/Language";

const BASE_URL = 'http://localhost:1234';
const TIMEOUT = 5000;

export const Members = {
    create: async (member: Member, lang: Language) => {
        let data = JSON.stringify({
            ...member,
            language: lang,
        });
        const res = await fetch(`${BASE_URL}/api/v1/admin/members`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
                'Access-Control-Allow-Origin': '*'
            },
            body: data,
            signal: AbortSignal.timeout(TIMEOUT),
        });
        return res.json();
    },
    uploadAvatar: async (memberId: string, file: File) => {
        const formData = new FormData();
        formData.append('avatar', file);
        return fetch(`${BASE_URL}/api/v1/admin/members/${memberId}/upload`, {
            method: 'POST',
            body: formData,
        });
    }
}