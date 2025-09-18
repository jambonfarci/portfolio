import { describe, it, expect, beforeEach, vi } from 'vitest';
import { get } from 'svelte/store';
import { projects, projectsLoading, featuredProjects, projectCategories, projectsStore } from './projects';
import { apiClient } from '$lib/api/client';
import type { Project } from '$lib/types';

// Mock the API client
vi.mock('$lib/api/client', () => ({
	apiClient: {
		getProjects: vi.fn(),
		getProject: vi.fn(),
		createProject: vi.fn(),
		updateProject: vi.fn(),
		deleteProject: vi.fn()
	}
}));

const mockApiClient = vi.mocked(apiClient);

describe('Projects Store', () => {
	const mockProjects: Project[] = [
		{
			id: 1,
			title: 'Project 1',
			description: 'Description 1',
			technologies: ['Rust'],
			category: 'Web',
			featured: true,
			created_at: '2024-01-01T00:00:00Z'
		},
		{
			id: 2,
			title: 'Project 2',
			description: 'Description 2',
			technologies: ['Svelte'],
			category: 'Frontend',
			featured: false,
			created_at: '2024-01-02T00:00:00Z'
		}
	];

	beforeEach(() => {
		// Reset stores
		projects.set([]);
		projectsLoading.set({ isLoading: false, error: null });
		
		// Clear all mocks
		vi.clearAllMocks();
	});

	describe('loadProjects', () => {
		it('should load projects successfully', async () => {
			mockApiClient.getProjects.mockResolvedValueOnce({
				success: true,
				data: mockProjects
			});

			await projectsStore.loadProjects();

			expect(get(projects)).toEqual(mockProjects);
			expect(get(projectsLoading)).toEqual({ isLoading: false, error: null });
		});

		it('should handle API errors', async () => {
			const error = { code: 'API_ERROR', message: 'Failed to fetch' };
			mockApiClient.getProjects.mockResolvedValueOnce({
				success: false,
				error
			});

			await projectsStore.loadProjects();

			expect(get(projects)).toEqual([]);
			expect(get(projectsLoading).error).toEqual(error);
		});

		it('should handle network errors', async () => {
			mockApiClient.getProjects.mockRejectedValueOnce(new Error('Network error'));

			await projectsStore.loadProjects();

			expect(get(projects)).toEqual([]);
			expect(get(projectsLoading).error?.code).toBe('NETWORK_ERROR');
		});

		it('should set loading state during request', async () => {
			let resolvePromise: (value: any) => void;
			const promise = new Promise<any>(resolve => {
				resolvePromise = resolve;
			});
			
			mockApiClient.getProjects.mockReturnValueOnce(promise as any);

			const loadPromise = projectsStore.loadProjects();
			
			// Check loading state is true during request
			expect(get(projectsLoading).isLoading).toBe(true);
			
			// Resolve the promise
			resolvePromise!({ success: true, data: mockProjects });
			await loadPromise;
			
			// Check loading state is false after request
			expect(get(projectsLoading).isLoading).toBe(false);
		});
	});

	describe('createProject', () => {
		it('should create project successfully', async () => {
			const newProject = mockProjects[0];
			mockApiClient.createProject.mockResolvedValueOnce({
				success: true,
				data: newProject
			});

			const result = await projectsStore.createProject({
				title: newProject.title,
				description: newProject.description,
				technologies: newProject.technologies,
				category: newProject.category
			});

			expect(result).toBe(true);
			expect(get(projects)).toContain(newProject);
		});
	});

	describe('derived stores', () => {
		beforeEach(() => {
			projects.set(mockProjects);
		});

		it('should filter featured projects', () => {
			const featured = get(featuredProjects);
			expect(featured).toHaveLength(1);
			expect(featured[0].featured).toBe(true);
		});

		it('should extract unique categories', () => {
			const categories = get(projectCategories);
			expect(categories).toEqual(['Frontend', 'Web']);
		});
	});
});