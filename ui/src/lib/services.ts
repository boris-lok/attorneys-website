import type {Member} from "$lib/models/Member";
import type {Language} from "$lib/models/Language";
import {from} from "rxjs";

const BASE_URL = "http://localhost:1234/api/v1";
const ADMIN_URL = `${BASE_URL}/admin`;
const TIMEOUT = 5000;

export const Members = {
    create: (member: Member, lang: Language) => {
        let data = JSON.stringify({
            ...member,
            language: lang,
        });

        const request: Promise<{ member_id: string }> = fetch(
            `${ADMIN_URL}/members`,
            {
                method: "POST",
                headers: {
                    "Content-Type": "application/json",
                },
                body: data,
                signal: AbortSignal.timeout(TIMEOUT),
            },
        ).then((res) => res.json());

        return from(request);
    },
    uploadAvatar: (memberId: string, file: File) => {
        const formData = new FormData();
        formData.append("avatar", file);

        const request: Promise<Response> = fetch(
            `${ADMIN_URL}/members/${memberId}/avatar`,
            {
                method: "POST",
                body: formData,
            },
        );

        return from(request);
    },
};
