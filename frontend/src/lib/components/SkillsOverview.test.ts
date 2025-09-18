import { describe, it, expect } from 'vitest';
import type { Skill } from '$lib/types';

// Mock skills data for testing
const mockSkillsByCategory: Record<string, Skill[]> = {
	'Frontend': [
		{
			id: 1,
			name: 'Svelte',
			category: 'Frontend',
			level: 5,
			years_experience: 3,
			description: 'Framework moderne pour interfaces utilisateur'
		},
		{
			id: 2,
			name: 'TypeScript',
			category: 'Frontend',
			level: 4,
			years_experience: 4
		}
	],
	'Backend': [
		{
			id: 3,
			name: 'Rust',
			category: 'Backend',
			level: 4,
			years_experience: 2,
			description: 'Langage système performant et sûr'
		},
		{
			id: 4,
			name: 'Node.js',
			category: 'Backend',
			level: 3,
			years_experience: 5
		}
	]
};

// Helper functions for testing component logic
function getSkillLevelDisplay(level: number): string {
	const levels = ['Débutant', 'Intermédiaire', 'Confirmé', 'Expert', 'Maître'];
	return levels[level - 1] || 'Débutant';
}

function getSkillLevelColor(level: number): string {
	const colors = [
		'bg-red-100 text-red-800',
		'bg-yellow-100 text-yellow-800', 
		'bg-blue-100 text-blue-800',
		'bg-green-100 text-green-800',
		'bg-purple-100 text-purple-800'
	];
	return colors[level - 1] || colors[0];
}

function getCategoryIcon(category: string): string {
	const icons: Record<string, string> = {
		'Frontend': '🎨',
		'Backend': '⚙️',
		'Database': '🗄️',
		'DevOps': '🚀',
		'Tools': '🔧',
		'Mobile': '📱',
		'Design': '✨'
	};
	return icons[category] || '💻';
}

describe('SkillsOverview Component Logic', () => {
	it('should have valid skills data structure', () => {
		expect(mockSkillsByCategory).toHaveProperty('Frontend');
		expect(mockSkillsByCategory).toHaveProperty('Backend');
		expect(Array.isArray(mockSkillsByCategory.Frontend)).toBe(true);
		expect(Array.isArray(mockSkillsByCategory.Backend)).toBe(true);
	});

	it('should validate skill properties', () => {
		const skill = mockSkillsByCategory.Frontend[0];
		expect(skill).toHaveProperty('id');
		expect(skill).toHaveProperty('name');
		expect(skill).toHaveProperty('category');
		expect(skill).toHaveProperty('level');
		expect(typeof skill.level).toBe('number');
		expect(skill.level).toBeGreaterThan(0);
		expect(skill.level).toBeLessThanOrEqual(5);
	});

	it('should correctly map skill levels to display text', () => {
		expect(getSkillLevelDisplay(1)).toBe('Débutant');
		expect(getSkillLevelDisplay(2)).toBe('Intermédiaire');
		expect(getSkillLevelDisplay(3)).toBe('Confirmé');
		expect(getSkillLevelDisplay(4)).toBe('Expert');
		expect(getSkillLevelDisplay(5)).toBe('Maître');
		expect(getSkillLevelDisplay(0)).toBe('Débutant'); // fallback
		expect(getSkillLevelDisplay(6)).toBe('Débutant'); // fallback
	});

	it('should correctly map skill levels to colors', () => {
		expect(getSkillLevelColor(1)).toBe('bg-red-100 text-red-800');
		expect(getSkillLevelColor(2)).toBe('bg-yellow-100 text-yellow-800');
		expect(getSkillLevelColor(3)).toBe('bg-blue-100 text-blue-800');
		expect(getSkillLevelColor(4)).toBe('bg-green-100 text-green-800');
		expect(getSkillLevelColor(5)).toBe('bg-purple-100 text-purple-800');
	});

	it('should correctly map categories to icons', () => {
		expect(getCategoryIcon('Frontend')).toBe('🎨');
		expect(getCategoryIcon('Backend')).toBe('⚙️');
		expect(getCategoryIcon('Database')).toBe('🗄️');
		expect(getCategoryIcon('DevOps')).toBe('🚀');
		expect(getCategoryIcon('Tools')).toBe('🔧');
		expect(getCategoryIcon('Mobile')).toBe('📱');
		expect(getCategoryIcon('Design')).toBe('✨');
		expect(getCategoryIcon('Unknown')).toBe('💻'); // fallback
	});

	it('should handle skills with optional properties', () => {
		const skillWithAllProps = mockSkillsByCategory.Frontend[0];
		expect(skillWithAllProps.years_experience).toBeDefined();
		expect(skillWithAllProps.description).toBeDefined();

		const skillWithoutDescription = mockSkillsByCategory.Frontend[1];
		expect(skillWithoutDescription.description).toBeUndefined();
	});

	it('should count skills correctly per category', () => {
		expect(mockSkillsByCategory.Frontend).toHaveLength(2);
		expect(mockSkillsByCategory.Backend).toHaveLength(2);
	});

	it('should handle empty skills data', () => {
		const emptySkills: Record<string, Skill[]> = {};
		expect(Object.keys(emptySkills)).toHaveLength(0);
	});

	it('should handle single skill category', () => {
		const singleSkillCategory = {
			'Database': [
				{
					id: 5,
					name: 'SQLite',
					category: 'Database',
					level: 3,
					years_experience: 2
				}
			]
		};
		
		expect(singleSkillCategory.Database).toHaveLength(1);
		expect(singleSkillCategory.Database[0].name).toBe('SQLite');
	});

	it('should validate skill level ranges', () => {
		const skills = Object.values(mockSkillsByCategory).flat();
		skills.forEach(skill => {
			expect(skill.level).toBeGreaterThanOrEqual(1);
			expect(skill.level).toBeLessThanOrEqual(5);
		});
	});
});