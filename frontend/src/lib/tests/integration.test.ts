import { describe, it, expect, beforeAll, afterAll } from 'vitest';
import { apiClient } from '$lib/api/client';
import type { Project, Skill, Profile, ContactMessage } from '$lib/types';

describe('API Integration Tests', () => {
	let testProject: Project;
	let testSkill: Skill;

	beforeAll(async () => {
		// Wait for backend to be ready
		let retries = 5;
		while (retries > 0) {
			try {
				const response = await fetch('http://localhost:3001/health');
				if (response.ok) break;
			} catch (error) {
				console.log('Waiting for backend...');
			}
			retries--;
			await new Promise(resolve => setTimeout(resolve, 1000));
		}
		
		if (retries === 0) {
			throw new Error('Backend not available for testing');
		}
	});

	describe('Profile API', () => {
		it('should fetch profile data', async () => {
			const response = await apiClient.getProfile();
			
			expect(response.success).toBe(true);
			expect(response.data).toBeDefined();
			expect(response.data?.name).toBeDefined();
			expect(response.data?.email).toBeDefined();
		});
	});

	describe('Projects API', () => {
		it('should fetch all projects', async () => {
			const response = await apiClient.getProjects();
			
			expect(response.success).toBe(true);
			expect(response.data).toBeDefined();
			expect(Array.isArray(response.data)).toBe(true);
		});

		it('should fetch featured projects', async () => {
			const response = await apiClient.getProjects(undefined, true);
			
			expect(response.success).toBe(true);
			expect(response.data).toBeDefined();
			expect(Array.isArray(response.data)).toBe(true);
			
			// All returned projects should be featured
			response.data?.forEach(project => {
				expect(project.featured).toBe(true);
			});
		});

		it('should fetch projects by category', async () => {
			const response = await apiClient.getProjects('web');
			
			expect(response.success).toBe(true);
			expect(response.data).toBeDefined();
			expect(Array.isArray(response.data)).toBe(true);
			
			// All returned projects should be in 'web' category
			response.data?.forEach(project => {
				expect(project.category).toBe('web');
			});
		});

		it('should fetch a specific project by ID', async () => {
			// First get all projects to get a valid ID
			const allProjectsResponse = await apiClient.getProjects();
			expect(allProjectsResponse.success).toBe(true);
			expect(allProjectsResponse.data?.length).toBeGreaterThan(0);
			
			const projectId = allProjectsResponse.data![0].id;
			const response = await apiClient.getProject(projectId);
			
			expect(response.success).toBe(true);
			expect(response.data).toBeDefined();
			expect(response.data?.id).toBe(projectId);
		});
	});

	describe('Skills API', () => {
		it('should fetch all skills', async () => {
			const response = await apiClient.getSkills();
			
			expect(response.success).toBe(true);
			expect(response.data).toBeDefined();
			expect(Array.isArray(response.data)).toBe(true);
		});

		it('should fetch skills by category', async () => {
			const response = await apiClient.getSkills('Backend');
			
			expect(response.success).toBe(true);
			expect(response.data).toBeDefined();
			expect(Array.isArray(response.data)).toBe(true);
			
			// All returned skills should be in 'Backend' category
			response.data?.forEach(skill => {
				expect(skill.category).toBe('Backend');
			});
		});
	});

	describe('Contact API', () => {
		it('should send a contact message', async () => {
			const message: ContactMessage = {
				name: 'Test User',
				email: 'test@example.com',
				subject: 'Test Message',
				message: 'This is a test message for integration testing purposes.'
			};

			const response = await apiClient.sendContactMessage(message);
			
			expect(response.success).toBe(true);
		});

		it('should validate contact message fields', async () => {
			const invalidMessage: ContactMessage = {
				name: '',
				email: 'invalid-email',
				subject: '',
				message: ''
			};

			const response = await apiClient.sendContactMessage(invalidMessage);
			
			expect(response.success).toBe(false);
			expect(response.error).toBeDefined();
		});
	});

	describe('Error Handling', () => {
		it('should handle 404 errors gracefully', async () => {
			const response = await apiClient.getProject(99999);
			
			expect(response.success).toBe(false);
			expect(response.error).toBeDefined();
			expect(response.error?.code).toBeDefined();
		});

		it('should handle network errors gracefully', async () => {
			// Create a client with invalid URL
			const invalidClient = new (apiClient.constructor as any)('http://localhost:9999');
			const response = await invalidClient.getProjects();
			
			expect(response.success).toBe(false);
			expect(response.error).toBeDefined();
			expect(response.error?.code).toBe('NETWORK_ERROR');
		});
	});
});