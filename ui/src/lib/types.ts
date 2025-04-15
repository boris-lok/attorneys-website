type NavItem = {
	icon: string
	name: string
}

type Link = NavItem & { url: string }
type IconButton = NavItem & { onClick: () => void }

export type NavigationItem = Link | IconButton

// The language enum that we support
export type Language = 'en' | 'zh'

// The request of creating home content
export type CreateHomeRequest = {
	data: string
	language: Language
	seq: number
}

// The request of updating home content
export type UpdateHomeRequest = CreateHomeRequest & { id: string }

// The data structure of home data
export type HomeData = {
	id: string
	language: Language
	data: {
		data: string
	}
}

// The data structure of service data
export type ServiceData = {
	id: string
	language: Language
	data: {
		title: string
		data: string
		icon?: string
	}
	seq: number
}

// The request of creating service content
export type CreateServiceRequest = {
	title: string
	data: string
	language: Language
	seq: number
}

// The request of updating service content
export type UpdateServiceRequest = { id: string } & CreateServiceRequest

// The request of creating member content
export type CreateMemberRequest = {
	name: string
	description: string
	language: Language
	seq: number
}

// The request of updating member content
export type UpdateMemberRequest = { id: string } & CreateMemberRequest

// The data structure of simple member
export type SimpleMember = {
	id: string
	name: string
	avatar?: string
	seq: number
}

// The data structure of image
export type ImageData = {
	large_image: string
	small_image: string
}

// The data structure of member
export type MemberData = {
	id: string
	language: Language
	data: {
		name: string
		description: string
	}
	avatar?: ImageData
	seq: number
}

// The request of creating article content
export type CreateArticleRequest = {
	title: string
	content: string
	language: Language
	seq: number
}

// The request of updating article content
export type UpdateArticleRequest = { id: string } & CreateArticleRequest

// The data structure of article
export type ArticleData = {
	id: string
	language: Language
	data: {
		title: string
		content: string
		category_id?: string
	}
	seq: number
}

export type SimpleArticle = {
	id: string
	title: string
	language: Language
	createdAt: Date
	createdAtString: string
	seq: number
}

export type CategoryData = {
	id: string
	language: Language
	data: {
		icon?: string
		name: string
	}
	seq: number
}

// The request of creating article content
export type CreateCategoryRequest = {
	icon?: string
	name: string
	language: Language
	seq: number
}

// The request of updating article content
export type UpdateCategoryRequest = { id: string } & CreateCategoryRequest
