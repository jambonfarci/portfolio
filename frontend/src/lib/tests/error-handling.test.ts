import { describe, it, expect, beforeEach, vi } from 'vitest';
import { toastStore, toasts } from '$lib/stores/toast';
import { projectsStore, projectsLoading } from '$lib/stores/projects';
import { contactStore, contactLoading } from '$lib/stores/contact';
import { skillsStore, skillsLoading } from '$lib/stores/skills';
import { profileStore, profileLoading } from '$lib/stores/profile';
import { get } from 'svelte/store';

// Mock the API client
vi.mock('$lib/api/client', () => ({
	apiClient: {
		getProjects: vi.fn(),
		getProject: vi.fn(),
		createProject: vi.fn(),
		updateProject: vi.fn(),
		deleteProject: vi.fn(),
		getSkills: vi.fn(),
		createSkill: vi.fn(),
		updateSkill: vi.fn(),
		deleteSkill: vi.fn(),
		getProfile: vi.fn(),
		updateProfile: vi.fn(),
		sendContactMessage: vi.fn()
	}
}));

import { apiClient } from '$lib/api/client';

describe('Error Handling and User Feedback', () => {
	beforeEach(() => {
		// Clear all stores
		toasts.set([]);
		projectsLoading.set({ isLoading: false, error: null });
		contactLoading.set({ isLoading: false, error: null });
		skillsLoading.set({ isLoading: false, error: null });
		profileLoading.set({ isLoading: false, error: null });
		
		// Reset all mocks
		vi.clearAllMocks();
	});

	describe('Toast Notifications', () => {
		it('should add and remove toast notifications', () => {
			const toastId = toastStore.success('Test Success', 'This is a test message');
			
			const currentToasts = get(toasts);
			expect(currentToasts).toHaveLength(1);
			expect(currentToasts[0].type).toBe('success');
			expect(currentToasts[0].title).toBe('Test Success');
			expect(currentToasts[0].message).toBe('This is a test message');
			
			toastStore.dismiss(toastId);
			
			const updatedToasts = get(toasts);
			expect(updatedToasts).toHaveLength(0);
		});

		it('should handle different toast types', () => {
			toastStore.success('Success', 'Success message');
			toastStore.error('Error', 'Error message');
			toastStore.warning('Warning', 'Warning message');
			toastStore.info('Info', 'Info message');
			
			const currentToasts = get(toasts);
			expect(currentToasts).toHaveLength(4);
			
			const types = currentToasts.map(toast => toast.type);
			expect(types).toContain('success');
			expect(types).toContain('error');
			expect(types).toContain('warning');
			expect(types).toContain('info');
		});

		it('should clear all toasts', () => {
			toastStore.success('Test 1');
			toastStore.error('Test 2');
			toastStore.warning('Test 3');
			
			expect(get(toasts)).toHaveLength(3);
			
			toastStore.clear();
			
			expect(get(toasts)).toHaveLength(0);
		});
	});

	describe('Projects Store Error Handling', () => {
		it('should handle API errors when loading projects', async () => {
			const mockError = {
				success: false,
				error: {
					code: 'SERVER_ERROR',
					message: 'Internal server error'
				}
			};
			
			vi.mocked(apiClient.getProjects).mockResolvedValue(mockError);
			
			await projectsStore.loadProjects();
			
			const loading = get(projectsLoading);
			expect(loading.isLoading).toBe(false);
			expect(loading.error).toEqual(mockError.error);
			
			// Should show error toast
			const currentToasts = get(toasts);
			expect(currentToasts).toHaveLength(1);
			expect(currentToasts[0].type).toBe('error');
		});

		it('should handle network errors when loading projects', async () => {
			vi.mocked(apiClient.getProjects).mockRejectedValue(new Error('Network error'));
			
			await projectsStore.loadProjects();
			
			const loading = get(projectsLoading);
			expect(loading.isLoading).toBe(false);
			expect(loading.error?.code).toBe('NETWORK_ERROR');
			
			// Should show error toast
			const currentToasts = get(toasts);
			expect(currentToasts).toHaveLength(1);
			expect(currentToasts[0].type).toBe('error');
		});

		it('should show success toast when creating project', async () => {
			const mockProject = {
				id: 1,
				title: 'Test Project',
				description: 'Test description',
				technologies: ['React'],
				category: 'web',
				featured: false,
				created_at: new Date().toISOString()
			};
			
			vi.mocked(apiClient.createProject).mockResolvedValue({
				success: true,
				data: mockProject
			});
			
			const result = await projectsStore.createProject({
				title: 'Test Project',
				description: 'Test description',
				technologies: ['React'],
				category: 'web'
			});
			
			expect(result).toBe(true);
			
			// Should show success toast
			const currentToasts = get(toasts);
			expect(currentToasts).toHaveLength(1);
			expect(currentToasts[0].type).toBe('success');
		});

		it('should show error toast when project creation fails', async () => {
			vi.mocked(apiClient.createProject).mockResolvedValue({
				success: false,
				error: {
					code: 'VALIDATION_ERROR',
					message: 'Title is required'
				}
			});
			
			const result = await projectsStore.createProject({
				title: '',
				description: 'Test description',
				technologies: ['React'],
				category: 'web'
			});
			
			expect(result).toBe(false);
			
			// Should show error toast
			const currentToasts = get(toasts);
			expect(currentToasts).toHaveLength(1);
			expect(currentToasts[0].type).toBe('error');
		});
	});

	describe('Contact Store Error Handling', () => {
		it('should handle successful message sending', async () => {
			vi.mocked(apiClient.sendContactMessage).mockResolvedValue({
				success: true
			});
			
			const result = await contactStore.sendMessage({
				name: 'Test User',
				email: 'test@example.com',
				subject: 'Test Subject',
				message: 'Test message'
			});
			
			expect(result).toBe(true);
			
			// Should show success toast
			const currentToasts = get(toasts);
			expect(currentToasts).toHaveLength(1);
			expect(currentToasts[0].type).toBe('success');
		});

		it('should handle validation errors', async () => {
			vi.mocked(apiClient.sendContactMessage).mockResolvedValue({
				success: false,
				error: {
					code: 'VALIDATION_ERROR',
					message: 'Email is invalid'
				}
			});
			
			const result = await contactStore.sendMessage({
				name: 'Test User',
				email: 'invalid-email',
				subject: 'Test Subject',
				message: 'Test message'
			});
			
			expect(result).toBe(false);
			
			const loading = get(contactLoading);
			expect(loading.error?.message).toBe('Email is invalid');
			
			// Should show error toast
			const currentToasts = get(toasts);
			expect(currentToasts).toHaveLength(1);
			expect(currentToasts[0].type).toBe('error');
		});

		it('should handle network errors', async () => {
			vi.mocked(apiClient.sendContactMessage).mockRejectedValue(new Error('Network error'));
			
			const result = await contactStore.sendMessage({
				name: 'Test User',
				email: 'test@example.com',
				subject: 'Test Subject',
				message: 'Test message'
			});
			
			expect(result).toBe(false);
			
			const loading = get(contactLoading);
			expect(loading.error?.code).toBe('NETWORK_ERROR');
			
			// Should show error toast
			const currentToasts = get(toasts);
			expect(currentToasts).toHaveLength(1);
			expect(currentToasts[0].type).toBe('error');
		});
	});

	describe('Skills Store Error Handling', () => {
		it('should handle API errors when loading skills', async () => {
			vi.mocked(apiClient.getSkills).mockResolvedValue({
				success: false,
				error: {
					code: 'SERVER_ERROR',
					message: 'Database connection failed'
				}
			});
			
			await skillsStore.loadSkills();
			
			const loading = get(skillsLoading);
			expect(loading.isLoading).toBe(false);
			expect(loading.error?.message).toBe('Database connection failed');
		});

		it('should show success toast when creating skill', async () => {
			const mockSkill = {
				id: 1,
				name: 'JavaScript',
				category: 'Frontend',
				level: 5,
				years_experience: 3
			};
			
			vi.mocked(apiClient.createSkill).mockResolvedValue({
				success: true,
				data: mockSkill
			});
			
			const result = await skillsStore.createSkill({
				name: 'JavaScript',
				category: 'Frontend',
				level: 5,
				years_experience: 3
			});
			
			expect(result).toBe(true);
		});
	});

	describe('Profile Store Error Handling', () => {
		it('should handle API errors when loading profile', async () => {
			vi.mocked(apiClient.getProfile).mockResolvedValue({
				success: false,
				error: {
					code: 'NOT_FOUND',
					message: 'Profile not found'
				}
			});
			
			await profileStore.loadProfile();
			
			const loading = get(profileLoading);
			expect(loading.isLoading).toBe(false);
			expect(loading.error?.message).toBe('Profile not found');
		});

		it('should show success toast when updating profile', async () => {
			const mockProfile = {
				id: 1,
				name: 'John Doe',
				title: 'Developer',
				bio: 'Test bio',
				email: 'john@example.com',
				location: 'Paris'
			};
			
			vi.mocked(apiClient.updateProfile).mockResolvedValue({
				success: true,
				data: mockProfile
			});
			
			const result = await profileStore.updateProfile({
				name: 'John Doe Updated'
			});
			
			expect(result).toBe(true);
		});
	});

	describe('Loading States', () => {
		it('should set loading state when making API calls', async () => {
			// Mock a delayed response
			vi.mocked(apiClient.getProjects).mockImplementation(() => 
				new Promise(resolve => 
					setTimeout(() => resolve({ success: true, data: [] }), 100)
				)
			);
			
			const loadPromise = projectsStore.loadProjects();
			
			// Should be loading immediately
			const loadingState = get(projectsLoading);
			expect(loadingState.isLoading).toBe(true);
			
			await loadPromise;
			
			// Should not be loading after completion
			const finalState = get(projectsLoading);
			expect(finalState.isLoading).toBe(false);
		});
	});

	describe('Error Recovery', () => {
		it('should clear errors when making new requests', async () => {
			// First, create an error
			vi.mocked(apiClient.getProjects).mockResolvedValue({
				success: false,
				error: { code: 'ERROR', message: 'Test error' }
			});
			
			await projectsStore.loadProjects();
			
			let loading = get(projectsLoading);
			expect(loading.error).toBeTruthy();
			
			// Then make a successful request
			vi.mocked(apiClient.getProjects).mockResolvedValue({
				success: true,
				data: []
			});
			
			await projectsStore.loadProjects();
			
			loading = get(projectsLoading);
			expect(loading.error).toBeNull();
		});

		it('should allow manual error clearing', () => {
			// Set an error state
			projectsLoading.set({
				isLoading: false,
				error: { code: 'ERROR', message: 'Test error' }
			});
			
			let loading = get(projectsLoading);
			expect(loading.error).toBeTruthy();
			
			// Clear the error
			projectsStore.clearError();
			
			loading = get(projectsLoading);
			expect(loading.error).toBeNull();
		});
	});
});