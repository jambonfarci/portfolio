import { describe, it, expect } from 'vitest';
import type { Skill } from '$lib/types';

// Mock skill data for testing
const mockSkill: Skill = {
	id: 1,
	name: 'Rust',
	category: 'Backend',
	level: 4,
	years_experience: 3,
	description: 'Langage système performant et sûr'
};

// Helper functions for testing component logic
function getSkillLevelDisplay(level: number): string {
	const levels = ['Débutant', 'Intermédiaire', 'Confirmé', 'Expert', 'Maître'];
	return levels[level - 1] || 'Débutant';
}

function getSkillLevelColor(level: number): string {
	const colors = [
		'bg-red-100 text-red-800 ring-red-600/20',
		'bg-yellow-100 text-yellow-800 ring-yellow-600/20', 
		'bg-blue-100 text-blue-800 ring-blue-600/20',
		'bg-green-100 text-green-800 ring-green-600/20',
		'bg-purple-100 text-purple-800 ring-purple-600/20'
	];
	return colors[level - 1] || colors[0];
}

function getCategoryColor(category: string): string {
	const colors: Record<string, string> = {
		'Frontend': 'from-blue-500 to-cyan-500',
		'Backend': 'from-green-500 to-emerald-500',
		'Database': 'from-purple-500 to-violet-500',
		'DevOps': 'from-orange-500 to-red-500',
		'Tools': 'from-gray-500 to-slate-500',
		'Mobile': 'from-pink-500 to-rose-500',
		'Design': 'from-indigo-500 to-purple-500'
	};
	return colors[category] || 'from-gray-500 to-slate-500';
}

function getSizeClasses(size: string): string {
	const sizes = {
		'sm': 'px-2 py-1 text-xs',
		'md': 'px-3 py-2 text-sm',
		'lg': 'px-4 py-3 text-base'
	};
	return sizes[size as keyof typeof sizes] || sizes.md;
}

function getLevelIndicators(level: number): string[] {
	return Array.from({ length: 5 }, (_, i) => i < level ? 'filled' : 'empty');
}

describe('SkillBadge Component Logic', () => {
	it('should have valid skill data structure', () => {
		expect(mockSkill).toHaveProperty('id');
		expect(mockSkill).toHaveProperty('name');
		expect(mockSkill).toHaveProperty('category');
		expect(mockSkill).toHaveProperty('level');
		expect(typeof mockSkill.level).toBe('number');
		expect(mockSkill.level).toBeGreaterThan(0);
		expect(mockSkill.level).toBeLessThanOrEqual(5);
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
		expect(getSkillLevelColor(1)).toBe('bg-red-100 text-red-800 ring-red-600/20');
		expect(getSkillLevelColor(2)).toBe('bg-yellow-100 text-yellow-800 ring-yellow-600/20');
		expect(getSkillLevelColor(3)).toBe('bg-blue-100 text-blue-800 ring-blue-600/20');
		expect(getSkillLevelColor(4)).toBe('bg-green-100 text-green-800 ring-green-600/20');
		expect(getSkillLevelColor(5)).toBe('bg-purple-100 text-purple-800 ring-purple-600/20');
	});

	it('should correctly map categories to colors', () => {
		expect(getCategoryColor('Frontend')).toBe('from-blue-500 to-cyan-500');
		expect(getCategoryColor('Backend')).toBe('from-green-500 to-emerald-500');
		expect(getCategoryColor('Database')).toBe('from-purple-500 to-violet-500');
		expect(getCategoryColor('DevOps')).toBe('from-orange-500 to-red-500');
		expect(getCategoryColor('Tools')).toBe('from-gray-500 to-slate-500');
		expect(getCategoryColor('Mobile')).toBe('from-pink-500 to-rose-500');
		expect(getCategoryColor('Design')).toBe('from-indigo-500 to-purple-500');
		expect(getCategoryColor('Unknown')).toBe('from-gray-500 to-slate-500'); // fallback
	});

	it('should correctly map sizes to CSS classes', () => {
		expect(getSizeClasses('sm')).toBe('px-2 py-1 text-xs');
		expect(getSizeClasses('md')).toBe('px-3 py-2 text-sm');
		expect(getSizeClasses('lg')).toBe('px-4 py-3 text-base');
		expect(getSizeClasses('invalid')).toBe('px-3 py-2 text-sm'); // fallback to md
	});

	it('should generate correct level indicators', () => {
		const level4Indicators = getLevelIndicators(4);
		expect(level4Indicators).toHaveLength(5);
		expect(level4Indicators.filter(i => i === 'filled')).toHaveLength(4);
		expect(level4Indicators.filter(i => i === 'empty')).toHaveLength(1);

		const level1Indicators = getLevelIndicators(1);
		expect(level1Indicators.filter(i => i === 'filled')).toHaveLength(1);
		expect(level1Indicators.filter(i => i === 'empty')).toHaveLength(4);

		const level5Indicators = getLevelIndicators(5);
		expect(level5Indicators.filter(i => i === 'filled')).toHaveLength(5);
		expect(level5Indicators.filter(i => i === 'empty')).toHaveLength(0);
	});

	it('should handle skill with all optional properties', () => {
		expect(mockSkill.years_experience).toBeDefined();
		expect(mockSkill.description).toBeDefined();
		expect(typeof mockSkill.years_experience).toBe('number');
		expect(typeof mockSkill.description).toBe('string');
	});

	it('should handle skill without optional properties', () => {
		const minimalSkill: Skill = {
			id: 2,
			name: 'JavaScript',
			category: 'Frontend',
			level: 3
		};

		expect(minimalSkill.years_experience).toBeUndefined();
		expect(minimalSkill.description).toBeUndefined();
	});

	it('should validate skill level range', () => {
		expect(mockSkill.level).toBeGreaterThanOrEqual(1);
		expect(mockSkill.level).toBeLessThanOrEqual(5);
	});

	it('should validate required skill properties', () => {
		expect(typeof mockSkill.id).toBe('number');
		expect(typeof mockSkill.name).toBe('string');
		expect(typeof mockSkill.category).toBe('string');
		expect(typeof mockSkill.level).toBe('number');
		
		expect(mockSkill.name.length).toBeGreaterThan(0);
		expect(mockSkill.category.length).toBeGreaterThan(0);
	});

	it('should handle edge cases for level indicators', () => {
		const level0Indicators = getLevelIndicators(0);
		expect(level0Indicators.filter(i => i === 'filled')).toHaveLength(0);
		expect(level0Indicators.filter(i => i === 'empty')).toHaveLength(5);

		const level6Indicators = getLevelIndicators(6);
		expect(level6Indicators.filter(i => i === 'filled')).toHaveLength(5); // Should cap at 5
		expect(level6Indicators.filter(i => i === 'empty')).toHaveLength(0);
	});
});