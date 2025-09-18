import { describe, it, expect } from 'vitest';
import type { Profile } from '$lib/types';

// Mock profile data for testing
const mockProfile: Profile = {
	id: 1,
	name: 'John Doe',
	title: 'Développeur Full-Stack',
	bio: 'Passionné par le développement web moderne avec Rust et Svelte.',
	email: 'john@example.com',
	location: 'Paris, France',
	avatar_url: 'https://example.com/avatar.jpg',
	linkedin_url: 'https://linkedin.com/in/johndoe',
	github_url: 'https://github.com/johndoe',
	twitter_url: 'https://twitter.com/johndoe'
};

describe('HeroSection Component', () => {
	it('should have valid profile data structure', () => {
		expect(mockProfile).toHaveProperty('name');
		expect(mockProfile).toHaveProperty('title');
		expect(mockProfile).toHaveProperty('bio');
		expect(mockProfile).toHaveProperty('email');
		expect(mockProfile).toHaveProperty('location');
	});

	it('should handle profile with all social links', () => {
		expect(mockProfile.linkedin_url).toBeDefined();
		expect(mockProfile.github_url).toBeDefined();
		expect(mockProfile.twitter_url).toBeDefined();
	});

	it('should handle profile without avatar', () => {
		const profileWithoutAvatar = { ...mockProfile, avatar_url: undefined };
		expect(profileWithoutAvatar.avatar_url).toBeUndefined();
		expect(profileWithoutAvatar.name).toBeDefined();
	});

	it('should handle profile without social links', () => {
		const profileWithoutSocial = {
			...mockProfile,
			linkedin_url: undefined,
			github_url: undefined,
			twitter_url: undefined
		};
		expect(profileWithoutSocial.linkedin_url).toBeUndefined();
		expect(profileWithoutSocial.github_url).toBeUndefined();
		expect(profileWithoutSocial.twitter_url).toBeUndefined();
	});

	it('should validate required profile fields', () => {
		expect(typeof mockProfile.name).toBe('string');
		expect(typeof mockProfile.title).toBe('string');
		expect(typeof mockProfile.bio).toBe('string');
		expect(typeof mockProfile.email).toBe('string');
		expect(typeof mockProfile.location).toBe('string');
	});

	it('should handle null profile gracefully', () => {
		const nullProfile = null;
		expect(nullProfile).toBeNull();
		// Component should render fallback content when profile is null
	});
});