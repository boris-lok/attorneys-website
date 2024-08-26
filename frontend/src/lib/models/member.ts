type SimpleUser = {
    id: string;
    name: string;
}

export type Avatar = SimpleUser & {
    url: string;
}

export type Member = Avatar & { tags: string[] };

export type MemberDetails = Member & {
    exp: string[];
    cert: string[];
    skill: string[];
    edu: string[];
    lang: string[];
    description: string;
};

export type CardType = "string" | "list";