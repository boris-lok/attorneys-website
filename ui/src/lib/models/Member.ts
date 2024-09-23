export type Member = {
	name: string;
	description: string;
};

type FileImage = { file?: File };

export type MemberPreview = Member & FileImage;

export type SimpleMember = {
	member_id: string;
	name: string;
	avatar?: string;
}

export type MemberDetail = SimpleMember & {
	content: string;
};
