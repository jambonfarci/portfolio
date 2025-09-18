import { describe, it, expect, beforeEach, vi } from 'vitest';
import { apiClient } from './client';
import type { Project, Skill, Profile, ContactMessage } from '$lib/types';

// Mock fetch globally
const mockFetch = vi.fn();
global.fetch = mockFetch;

describe('ApiClient', () => {
	beforeEach(() => {
		mockFetch.mockClear();
	});

	describe('Projects API', () => {
		it('should fetch all projects', async () => {
			const mockProjects: Project[] = [
				{
					id: 1,
					title: 'Test Project',
					description: 'A test project',
					technologies: ['Rust', 'Svelte'],
					category: 'Web',
					featured: true,
					created_at: '2024-01-01T00:00:00Z'
				}
			];

			mockFetch.mockResolvedValueOnce({
				ok: true,
				json: async () => ({ success: true, data: mockProjects })
			});

			const result = await apiClient.getProjects();

			expect(mockFetch).toHaveBeenCalledWith(
				'http://localhost:3001/api/projects',
				expect.objectContaining({
					method: 'GET',
					headers: { 'Content-Type': 'application/json' }
				})
			);
			expect(result.success).toBe(true);
			expect(result.data).toEqual(mockProjects);
		});

		it('should fetch projects with filters', async () => {
			mockFetch.mockResolvedValueOnce({
				ok: true,
				json: async () => ({ success: true, data: [] })
			});

			await apiClient.getProjects('Web', true);

			expect(mockFetch).toHaveBeenCalledWith(
				'http://localhost:3001/api/projects?category=Web&featured=true',
				expect.any(Object)
			);
		});

		it('should handle network errors', async () => {
			mockFetch.mockRejectedValueOnce(new Error('Network error'));

			const result = await apiClient.getProjects();

			expect(result.success).toBe(false);
			expect(result.error?.code).toBe('NETWORK_ERROR');
		});
	});

	describe('Skills API', () => {
		it('should fetch all skills', async () => {
			const mockSkills: Skill[] = [
				{
					id: 1,
					name: 'Rust',
					category: 'Backend',
					level: 5
				}
			];

			mockFetch.mockResolvedValueOnce({
				ok: true,
				json: async () => ({ success: true, data: mockSkills })
			});

			const result = await apiClient.getSkills();

			expect(mockFetch).toHaveBeenCalledWith(
				'http://localhost:3001/api/skills',
				expect.objectContaining({
					method: 'GET'
				})
			);
			expect(result.success).toBe(true);
			expect(result.data).toEqual(mockSkills);
		});

		it('should fetch skills by category', async () => {
			mockFetch.mockResolvedValueOnce({
				ok: true,
				json: async () => ({ success: true, data: [] })
			});

			await apiClient.getSkills('Frontend');

			expect(mockFetch).toHaveBeenCalledWith(
				'http://localhost:3001/api/skills?category=Frontend',
				expect.any(Object)
			);
		});
	});

	describe('Profile API', () => {
		it('should fetch profile', async () => {
			const mockProfile: Profile = {
				id: 1,
				name: 'John Doe',
				title: 'Developer',
				bio: 'A developer',
				email: 'john@example.com',
				location: 'Paris'
			};

			mockFetch.mockResolvedValueOnce({
				ok: true,
				json: async () => ({ success: true, data: mockProfile })
			});

			const result = await apiClient.getProfile();

			expect(mockFetch).toHaveBeenCalledWith(
				'http://localhost:3001/api/profile',
				expect.objectContaining({
					method: 'GET'
				})
			);
			expect(result.success).toBe(true);
			expect(result.data).toEqual(mockProfile);
		});
	});

	describe('Contact API', () => {
		it('should send contact message', async () => {
			const message: ContactMessage = {
				name: 'Jane Doe',
				email: 'jane@example.com',
				subject: 'Test',
				message: 'Hello'
			};

			mockFetch.mockResolvedValueOnce({
				ok: true,
				json: async () => ({ success: true })
			});

			const result = await apiClient.sendContactMessage(message);

			expect(mockFetch).toHaveBeenCalledWith(
				'http://localhost:3001/api/contact',
				expect.objectContaining({
					method: 'POST',
					body: JSON.stringify(message)
				})
			);
			expect(result.success).toBe(true);
		});
	});

	describe('Error handling', () => {
		it('should handle HTTP error responses', async () => {
			mockFetch.mockResolvedValueOnce({
				ok: false,
				status: 404,
				statusText: 'Not Found',
				json: async () => ({
					success: false,
					error: { code: 'NOT_FOUND', message: 'Resource not found' }
				})
			});

			const result = await apiClient.getProjects();

			expect(result.success).toBe(false);
			expect(result.error?.code).toBe('NOT_FOUND');
		});

		it('should handle malformed responses', async () => {
			mockFetch.mockResolvedValueOnce({
				ok: false,
				status: 500,
				statusText: 'Internal Server Error',
				json: async () => ({})
			});

			const result = await apiClient.getProjects();

			expect(result.success).toBe(false);
			expect(result.error?.code).toBe('HTTP_ERROR');
		});
	});
});