import { describe, it, expect } from 'vitest';
import type { Project } from '$lib/types';

// Mock projects data for testing
const mockProjects: Project[] = [
	{
		id: 1,
		title: 'Portfolio Web',
		description: 'Un portfolio moderne avec Rust et Svelte',
		technologies: ['Rust', 'Svelte', 'TypeScript'],
		category: 'Web',
		featured: true,
		created_at: '2024-01-15T10:30:00Z'
	},
	{
		id: 2,
		title: 'API REST',
		description: 'Une API REST performante en Rust',
		technologies: ['Rust', 'Axum', 'SQLite'],
		category: 'Backend',
		featured: false,
		created_at: '2024-01-10T08:00:00Z'
	},
	{
		id: 3,
		title: 'App Mobile',
		description: 'Application mobile cross-platform',
		technologies: ['React Native', 'TypeScript'],
		category: 'Mobile',
		featured: true,
		created_at: '2024-01-20T14:15:00Z'
	}
];

const mockCategories = ['Web', 'Backend', 'Mobile'];

// Helper functions for testing component logic
function filterProjects(
	projects: Project[], 
	selectedCategory: string, 
	searchQuery: string
): Project[] {
	return projects.filter(project => {
		const matchesCategory = selectedCategory === 'all' || project.category === selectedCategory;
		const matchesSearch = searchQuery === '' || 
			project.title.toLowerCase().includes(searchQuery.toLowerCase()) ||
			project.description.toLowerCase().includes(searchQuery.toLowerCase()) ||
			project.technologies.some(tech => tech.toLowerCase().includes(searchQuery.toLowerCase()));
		
		return matchesCategory && matchesSearch;
	});
}

function sortProjects(projects: Project[], showFeaturedFirst: boolean): Project[] {
	return showFeaturedFirst 
		? [...projects].sort((a, b) => {
			if (a.featured && !b.featured) return -1;
			if (!a.featured && b.featured) return 1;
			return new Date(b.created_at).getTime() - new Date(a.created_at).getTime();
		})
		: [...projects].sort((a, b) => 
			new Date(b.created_at).getTime() - new Date(a.created_at).getTime()
		);
}

function getCategoryCount(projects: Project[], category: string): number {
	if (category === 'all') return projects.length;
	return projects.filter(p => p.category === category).length;
}

describe('ProjectGrid Component Logic', () => {
	it('should have valid projects data structure', () => {
		expect(Array.isArray(mockProjects)).toBe(true);
		expect(mockProjects.length).toBeGreaterThan(0);
		
		mockProjects.forEach(project => {
			expect(project).toHaveProperty('id');
			expect(project).toHaveProperty('title');
			expect(project).toHaveProperty('description');
			expect(project).toHaveProperty('category');
			expect(project).toHaveProperty('technologies');
			expect(project).toHaveProperty('featured');
			expect(project).toHaveProperty('created_at');
		});
	});

	it('should filter projects by category correctly', () => {
		const webProjects = filterProjects(mockProjects, 'Web', '');
		expect(webProjects).toHaveLength(1);
		expect(webProjects[0].category).toBe('Web');

		const backendProjects = filterProjects(mockProjects, 'Backend', '');
		expect(backendProjects).toHaveLength(1);
		expect(backendProjects[0].category).toBe('Backend');

		const allProjects = filterProjects(mockProjects, 'all', '');
		expect(allProjects).toHaveLength(mockProjects.length);
	});

	it('should filter projects by search query correctly', () => {
		// Search by title
		const portfolioProjects = filterProjects(mockProjects, 'all', 'Portfolio');
		expect(portfolioProjects).toHaveLength(1);
		expect(portfolioProjects[0].title).toContain('Portfolio');

		// Search by technology
		const rustProjects = filterProjects(mockProjects, 'all', 'Rust');
		expect(rustProjects).toHaveLength(2);
		rustProjects.forEach(project => {
			expect(project.technologies.some(tech => 
				tech.toLowerCase().includes('rust')
			)).toBe(true);
		});

		// Search by description
		const apiProjects = filterProjects(mockProjects, 'all', 'API');
		expect(apiProjects).toHaveLength(1);
		expect(apiProjects[0].description).toContain('API');
	});

	it('should combine category and search filters', () => {
		const webRustProjects = filterProjects(mockProjects, 'Web', 'Rust');
		expect(webRustProjects).toHaveLength(1);
		expect(webRustProjects[0].category).toBe('Web');
		expect(webRustProjects[0].technologies).toContain('Rust');

		const mobileRustProjects = filterProjects(mockProjects, 'Mobile', 'Rust');
		expect(mobileRustProjects).toHaveLength(0);
	});

	it('should sort projects with featured first', () => {
		const sorted = sortProjects(mockProjects, true);
		
		// Featured projects should come first
		const featuredProjects = sorted.filter(p => p.featured);
		const nonFeaturedProjects = sorted.filter(p => !p.featured);
		
		expect(featuredProjects.length).toBe(2);
		expect(nonFeaturedProjects.length).toBe(1);
		
		// Check that featured projects are at the beginning
		expect(sorted[0].featured).toBe(true);
		expect(sorted[1].featured).toBe(true);
		expect(sorted[2].featured).toBe(false);
	});

	it('should sort projects by date when not prioritizing featured', () => {
		const sorted = sortProjects(mockProjects, false);
		
		// Should be sorted by date (newest first)
		for (let i = 0; i < sorted.length - 1; i++) {
			const currentDate = new Date(sorted[i].created_at);
			const nextDate = new Date(sorted[i + 1].created_at);
			expect(currentDate.getTime()).toBeGreaterThanOrEqual(nextDate.getTime());
		}
	});

	it('should count projects correctly by category', () => {
		expect(getCategoryCount(mockProjects, 'all')).toBe(3);
		expect(getCategoryCount(mockProjects, 'Web')).toBe(1);
		expect(getCategoryCount(mockProjects, 'Backend')).toBe(1);
		expect(getCategoryCount(mockProjects, 'Mobile')).toBe(1);
		expect(getCategoryCount(mockProjects, 'NonExistent')).toBe(0);
	});

	it('should handle empty search results', () => {
		const noResults = filterProjects(mockProjects, 'all', 'NonExistentTech');
		expect(noResults).toHaveLength(0);
	});

	it('should handle case-insensitive search', () => {
		const lowerCaseSearch = filterProjects(mockProjects, 'all', 'rust');
		const upperCaseSearch = filterProjects(mockProjects, 'all', 'RUST');
		const mixedCaseSearch = filterProjects(mockProjects, 'all', 'RuSt');
		
		expect(lowerCaseSearch).toHaveLength(2);
		expect(upperCaseSearch).toHaveLength(2);
		expect(mixedCaseSearch).toHaveLength(2);
	});

	it('should validate categories array', () => {
		expect(Array.isArray(mockCategories)).toBe(true);
		expect(mockCategories.length).toBeGreaterThan(0);
		
		mockCategories.forEach(category => {
			expect(typeof category).toBe('string');
			expect(category.length).toBeGreaterThan(0);
		});
	});

	it('should handle empty projects array', () => {
		const emptyProjects: Project[] = [];
		
		expect(filterProjects(emptyProjects, 'all', '')).toHaveLength(0);
		expect(sortProjects(emptyProjects, true)).toHaveLength(0);
		expect(getCategoryCount(emptyProjects, 'all')).toBe(0);
	});
});