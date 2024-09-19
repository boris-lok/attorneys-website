import type {Member} from "$lib/models/Member";
import type {Language} from "$lib/models/Language";
import {from, of} from "rxjs";

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

export const Home = {
    list: () => {
        const content = `# 關於本所
### 義修法律事務所，擁有多年訴訟執業與非訟辦理之經驗，專精民事訴訟、不動產爭議、公司商務爭端、稅務、商標智財、家事案件與刑事辯護等案件，致力於為客戶提供高品質之全方位法律服務，並時時掌握最新的產業趨勢與法律動態。

### 本所專業團隊提供優質之地區性及國際性訴訟，以及商標、公司設立、併購、資產規劃，盡職調查、智慧財產權、公司及勞動事務等法律協助及相關諮詢服務。我們的使命是以法律專業和全心全意服務，以協助客戶降低成本，減低風險，減免問題之發生，有效解決糾紛，維護客戶正當權益，達成企業營運目標。

### 發揮個人能力並親力親為，心繫公義同時實踐法治，是義修法律事務所最引以為傲之特色。本所將持續以最高效率與最高品質．為客戶提供周延之法律服務。`;

        return of(content)
    }
}
