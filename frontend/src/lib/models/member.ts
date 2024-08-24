export type User = {
    id: string;
    name: string;
}

export type Avatar = User & {
    url: string;
}