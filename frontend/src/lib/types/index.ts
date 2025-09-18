// Type definitions for the portfolio application

export interface Project {
	id: number;
	title: string;
	description: string;
	long_description?: string;
	technologies: string[];
	github_url?: string;
	demo_url?: string;
	image_url?: string;
	category: string;
	featured: boolean;
	created_at: string;
}

export interface CreateProject {
	title: string;
	description: string;
	long_description?: string;
	technologies: string[];
	github_url?: string;
	demo_url?: string;
	image_url?: string;
	category: string;
	featured?: boolean;
}

export interface UpdateProject extends Partial<CreateProject> {}

export interface Skill {
	id: number;
	name: string;
	category: string;
	level: number; // 1-5 scale
	years_experience?: number;
	description?: string;
}

export interface CreateSkill {
	name: string;
	category: string;
	level: number;
	years_experience?: number;
	description?: string;
}

export interface UpdateSkill extends Partial<CreateSkill> {}

export interface Profile {
	id: number;
	name: string;
	title: string;
	bio: string;
	email: string;
	phone?: string;
	location: string;
	linkedin_url?: string;
	github_url?: string;
	twitter_url?: string;
}

export interface UpdateProfile extends Partial<Omit<Profile, 'id'>> {}

export interface ContactMessage {
	name: string;
	email: string;
	subject: string;
	message: string;
}

export interface ApiError {
	code: string;
	message: string;
	details?: string[];
}

export interface LoadingState {
	isLoading: boolean;
	error: ApiError | null;
}