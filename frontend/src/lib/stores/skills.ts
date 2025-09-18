import { writable, derived } from 'svelte/store';
import type { Skill, CreateSkill, UpdateSkill, LoadingState } from '$lib/types';
import { apiClient } from '$lib/api/client';

// Store for all skills
export const skills = writable<Skill[]>([]);

// Loading state for skills
export const skillsLoading = writable<LoadingState>({
	isLoading: false,
	error: null
});

// Derived store for skill categories
export const skillCategories = derived(skills, ($skills) => {
	const categories = new Set($skills.map(skill => skill.category));
	return Array.from(categories).sort();
});

// Derived store for skills grouped by category
export const skillsByCategory = derived(skills, ($skills) => {
	const grouped: Record<string, Skill[]> = {};
	
	$skills.forEach(skill => {
		if (!grouped[skill.category]) {
			grouped[skill.category] = [];
		}
		grouped[skill.category].push(skill);
	});
	
	// Sort skills within each category by level (descending) then by name
	Object.keys(grouped).forEach(category => {
		grouped[category].sort((a, b) => {
			if (a.level !== b.level) {
				return b.level - a.level; // Higher level first
			}
			return a.name.localeCompare(b.name);
		});
	});
	
	return grouped;
});

// Store actions
export const skillsStore = {
	// Load all skills
	async loadSkills(category?: string) {
		skillsLoading.update(state => ({ ...state, isLoading: true, error: null }));
		
		try {
			const response = await apiClient.getSkills(category);
			
			if (response.success && response.data) {
				skills.set(response.data);
				skillsLoading.update(state => ({ ...state, isLoading: false }));
			} else {
				skillsLoading.update(state => ({
					...state,
					isLoading: false,
					error: response.error || { code: 'UNKNOWN_ERROR', message: 'Unknown error occurred' }
				}));
			}
		} catch (error) {
			skillsLoading.update(state => ({
				...state,
				isLoading: false,
				error: { code: 'NETWORK_ERROR', message: 'Failed to load skills' }
			}));
		}
	},

	// Create a new skill
	async createSkill(skillData: CreateSkill): Promise<boolean> {
		try {
			const response = await apiClient.createSkill(skillData);
			
			if (response.success && response.data) {
				// Add the new skill to the store
				skills.update(currentSkills => [...currentSkills, response.data!]);
				return true;
			} else {
				console.error('Failed to create skill:', response.error);
				return false;
			}
		} catch (error) {
			console.error('Failed to create skill:', error);
			return false;
		}
	},

	// Update an existing skill
	async updateSkill(id: number, skillData: UpdateSkill): Promise<boolean> {
		try {
			const response = await apiClient.updateSkill(id, skillData);
			
			if (response.success && response.data) {
				// Update the skill in the store
				skills.update(currentSkills =>
					currentSkills.map(skill =>
						skill.id === id ? response.data! : skill
					)
				);
				return true;
			} else {
				console.error('Failed to update skill:', response.error);
				return false;
			}
		} catch (error) {
			console.error('Failed to update skill:', error);
			return false;
		}
	},

	// Delete a skill
	async deleteSkill(id: number): Promise<boolean> {
		try {
			const response = await apiClient.deleteSkill(id);
			
			if (response.success) {
				// Remove the skill from the store
				skills.update(currentSkills =>
					currentSkills.filter(skill => skill.id !== id)
				);
				return true;
			} else {
				console.error('Failed to delete skill:', response.error);
				return false;
			}
		} catch (error) {
			console.error('Failed to delete skill:', error);
			return false;
		}
	},

	// Clear error state
	clearError() {
		skillsLoading.update(state => ({ ...state, error: null }));
	}
};