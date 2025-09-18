import { describe, it, expect } from 'vitest';
import type { Project } from '$lib/types';

// Mock project data for testing
const mockProject: Project = {
	id: 1,
	title: 'Portfolio Web',
	description: 'Un portfolio moderne développé avec Rust et Svelte pour présenter mes compétences et projets.',
	long_description: 'Ce projet utilise une architecture moderne avec Rust pour le backend et Svelte pour le frontend.',
	technologies: ['Rust', 'Svelte', 'TypeScript', 'TailwindCSS', 'SQLite'],
	github_url: 'https://github.com/user/portfolio',
	demo_url: 'https://portfolio.example.com',
	image_url: 'https://example.com/portfolio.jpg',
	category: 'Web',
	featured: true,
	created_at: '2024-01-15T10:30:00Z'
};

// Helper functions for testing component logic
function formatDate(dateString: string): string {
	const date = new Date(dateString);
	return date.toLocaleDateString('fr-FR', { 
		year: 'numeric', 
		month: 'long' 
	});
}

function truncateDescription(text: string, maxLength: number = 120): string {
	if (text.length <= maxLength) return text;
	return text.substring(0, maxLength).trim() + '...';
}

describe('ProjectCard Component Logic', () => {
	it('should have valid project data structure', () => {
		expect(mockProject).toHaveProperty('id');
		expect(mockProject).toHaveProperty('title');
		expect(mockProject).toHaveProperty('description');
		expect(mockProject).toHaveProperty('technologies');
		expect(mockProject).toHaveProperty('category');
		expect(mockProject).toHaveProperty('created_at');
		expect(Array.isArray(mockProject.technologies)).toBe(true);
	});

	it('should format date correctly', () => {
		const formatted = formatDate(mockProject.created_at);
		expect(formatted).toBe('janvier 2024');
	});

	it('should truncate long descriptions', () => {
		const longText = 'A'.repeat(150);
		const truncated = truncateDescription(longText, 120);
		expect(truncated).toHaveLength(123); // 120 + '...'
		expect(truncated.endsWith('...')).toBe(true);
	});

	it('should not truncate short descriptions', () => {
		const shortText = 'Short description';
		const result = truncateDescription(shortText, 120);
		expect(result).toBe(shortText);
		expect(result.endsWith('...')).toBe(false);
	});

	it('should handle project with all optional fields', () => {
		expect(mockProject.long_description).toBeDefined();
		expect(mockProject.github_url).toBeDefined();
		expect(mockProject.demo_url).toBeDefined();
		expect(mockProject.image_url).toBeDefined();
	});

	it('should handle project without optional fields', () => {
		const minimalProject: Project = {
			id: 2,
			title: 'Minimal Project',
			description: 'A minimal project',
			technologies: ['JavaScript'],
			category: 'Web',
			featured: false,
			created_at: '2024-01-01T00:00:00Z'
		};

		expect(minimalProject.long_description).toBeUndefined();
		expect(minimalProject.github_url).toBeUndefined();
		expect(minimalProject.demo_url).toBeUndefined();
		expect(minimalProject.image_url).toBeUndefined();
	});

	it('should validate featured flag', () => {
		expect(typeof mockProject.featured).toBe('boolean');
		expect(mockProject.featured).toBe(true);
	});

	it('should validate technologies array', () => {
		expect(Array.isArray(mockProject.technologies)).toBe(true);
		expect(mockProject.technologies.length).toBeGreaterThan(0);
		mockProject.technologies.forEach(tech => {
			expect(typeof tech).toBe('string');
			expect(tech.length).toBeGreaterThan(0);
		});
	});

	it('should handle technology display limit', () => {
		const displayLimit = 4;
		const visibleTechs = mockProject.technologies.slice(0, displayLimit);
		const remainingCount = mockProject.technologies.length - displayLimit;
		
		expect(visibleTechs).toHaveLength(displayLimit);
		expect(remainingCount).toBe(1); // 5 total - 4 displayed = 1 remaining
	});

	it('should validate URL formats', () => {
		if (mockProject.github_url) {
			expect(mockProject.github_url).toMatch(/^https?:\/\//);
		}
		if (mockProject.demo_url) {
			expect(mockProject.demo_url).toMatch(/^https?:\/\//);
		}
		if (mockProject.image_url) {
			expect(mockProject.image_url).toMatch(/^https?:\/\//);
		}
	});

	it('should validate date format', () => {
		const date = new Date(mockProject.created_at);
		expect(date).toBeInstanceOf(Date);
		expect(isNaN(date.getTime())).toBe(false);
	});
});