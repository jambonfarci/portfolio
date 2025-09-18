// API client for backend communication
import type { 
	Project, 
	CreateProject, 
	UpdateProject, 
	Skill, 
	CreateSkill, 
	UpdateSkill, 
	Profile, 
	UpdateProfile, 
	ContactMessage 
} from '$lib/types';

const API_BASE_URL = import.meta.env.VITE_API_URL || 'http://localhost:3001';

export interface ApiResponse<T> {
	success: boolean;
	data?: T;
	error?: {
		code: string;
		message: string;
		details?: string[];
	};
}

class ApiClient {
	private baseUrl: string;

	constructor(baseUrl: string = API_BASE_URL) {
		this.baseUrl = baseUrl;
	}

	private async request<T>(
		endpoint: string,
		options: RequestInit = {}
	): Promise<ApiResponse<T>> {
		const url = `${this.baseUrl}${endpoint}`;
		
		try {
			const response = await fetch(url, {
				headers: {
					'Content-Type': 'application/json',
					...options.headers,
				},
				...options,
			});

			const data = await response.json();
			
			// Handle HTTP error status codes
			if (!response.ok) {
				return {
					success: false,
					error: data.error || {
						code: 'HTTP_ERROR',
						message: `HTTP ${response.status}: ${response.statusText}`,
					},
				};
			}

			return data;
		} catch (error) {
			return {
				success: false,
				error: {
					code: 'NETWORK_ERROR',
					message: 'Failed to connect to server',
				},
			};
		}
	}

	// Generic HTTP methods
	async get<T>(endpoint: string): Promise<ApiResponse<T>> {
		return this.request<T>(endpoint, { method: 'GET' });
	}

	async post<T>(endpoint: string, data: any): Promise<ApiResponse<T>> {
		return this.request<T>(endpoint, {
			method: 'POST',
			body: JSON.stringify(data),
		});
	}

	async put<T>(endpoint: string, data: any): Promise<ApiResponse<T>> {
		return this.request<T>(endpoint, {
			method: 'PUT',
			body: JSON.stringify(data),
		});
	}

	async delete<T>(endpoint: string): Promise<ApiResponse<T>> {
		return this.request<T>(endpoint, { method: 'DELETE' });
	}

	// Project API methods
	async getProjects(category?: string, featured?: boolean): Promise<ApiResponse<Project[]>> {
		let endpoint = '/api/projects';
		const params = new URLSearchParams();
		
		if (category) params.append('category', category);
		if (featured !== undefined) params.append('featured', featured.toString());
		
		if (params.toString()) {
			endpoint += `?${params.toString()}`;
		}
		
		return this.get<Project[]>(endpoint);
	}

	async getProject(id: number): Promise<ApiResponse<Project>> {
		return this.get<Project>(`/api/projects/${id}`);
	}

	async createProject(project: CreateProject): Promise<ApiResponse<Project>> {
		return this.post<Project>('/api/projects', project);
	}

	async updateProject(id: number, project: UpdateProject): Promise<ApiResponse<Project>> {
		return this.put<Project>(`/api/projects/${id}`, project);
	}

	async deleteProject(id: number): Promise<ApiResponse<void>> {
		return this.delete<void>(`/api/projects/${id}`);
	}

	// Skills API methods
	async getSkills(category?: string): Promise<ApiResponse<Skill[]>> {
		let endpoint = '/api/skills';
		if (category) {
			endpoint += `?category=${encodeURIComponent(category)}`;
		}
		return this.get<Skill[]>(endpoint);
	}

	async createSkill(skill: CreateSkill): Promise<ApiResponse<Skill>> {
		return this.post<Skill>('/api/skills', skill);
	}

	async updateSkill(id: number, skill: UpdateSkill): Promise<ApiResponse<Skill>> {
		return this.put<Skill>(`/api/skills/${id}`, skill);
	}

	async deleteSkill(id: number): Promise<ApiResponse<void>> {
		return this.delete<void>(`/api/skills/${id}`);
	}

	// Profile API methods
	async getProfile(): Promise<ApiResponse<Profile>> {
		return this.get<Profile>('/api/profile');
	}

	async updateProfile(profile: UpdateProfile): Promise<ApiResponse<Profile>> {
		return this.put<Profile>('/api/profile', profile);
	}

	// Contact API methods
	async sendContactMessage(message: ContactMessage): Promise<ApiResponse<void>> {
		return this.post<void>('/api/contact', message);
	}
}

export const apiClient = new ApiClient();