import { describe, it, expect, beforeEach, vi } from 'vitest';
import { get } from 'svelte/store';
import { skills, skillsLoading, skillCategories, skillsByCategory, skillsStore } from './skills';
import { apiClient } from '$lib/api/client';
import type { Skill } from '$lib/types';

// Mock the API client
vi.mock('$lib/api/client', () => ({
	apiClient: {
		getSkills: vi.fn(),
		createSkill: vi.fn(),
		updateSkill: vi.fn(),
		deleteSkill: vi.fn()
	}
}));

const mockApiClient = vi.mocked(apiClient);

describe('Skills Store', () => {
	const mockSkills: Skill[] = [
		{
			id: 1,
			name: 'Rust',
			category: 'Backend',
			level: 5
		},
		{
			id: 2,
			name: 'Svelte',
			category: 'Frontend',
			level: 4
		},
		{
			id: 3,
			name: 'TypeScript',
			category: 'Frontend',
			level: 5
		}
	];

	beforeEach(() => {
		// Reset stores
		skills.set([]);
		skillsLoading.set({ isLoading: false, error: null });
		
		// Clear all mocks
		vi.clearAllMocks();
	});

	describe('loadSkills', () => {
		it('should load skills successfully', async () => {
			mockApiClient.getSkills.mockResolvedValueOnce({
				success: true,
				data: mockSkills
			});

			await skillsStore.loadSkills();

			expect(get(skills)).toEqual(mockSkills);
			expect(get(skillsLoading)).toEqual({ isLoading: false, error: null });
		});

		it('should handle API errors', async () => {
			const error = { code: 'API_ERROR', message: 'Failed to fetch' };
			mockApiClient.getSkills.mockResolvedValueOnce({
				success: false,
				error
			});

			await skillsStore.loadSkills();

			expect(get(skills)).toEqual([]);
			expect(get(skillsLoading).error).toEqual(error);
		});
	});

	describe('derived stores', () => {
		beforeEach(() => {
			skills.set(mockSkills);
		});

		it('should extract unique categories', () => {
			const categories = get(skillCategories);
			expect(categories).toEqual(['Backend', 'Frontend']);
		});

		it('should group skills by category and sort them', () => {
			const grouped = get(skillsByCategory);
			
			expect(grouped.Backend).toHaveLength(1);
			expect(grouped.Frontend).toHaveLength(2);
			
			// Should be sorted by level (desc) then name
			expect(grouped.Frontend[0].name).toBe('TypeScript'); // Level 5
			expect(grouped.Frontend[1].name).toBe('Svelte'); // Level 4
		});
	});
});