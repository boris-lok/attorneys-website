type Member = {
  name: string;
  description: string;
};

type FileImage = { file?: File };

export type MemberPreview = Member & FileImage;
