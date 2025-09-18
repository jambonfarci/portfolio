import { describe, it, expect } from 'vitest';
import type { ContactMessage } from '$lib/types';

// Mock contact message data for testing
const mockContactMessage: ContactMessage = {
	name: 'John Doe',
	email: 'john@example.com',
	subject: 'Demande de collaboration',
	message: 'Bonjour, je souhaiterais discuter d\'un projet de développement web avec vous.'
};

// Helper functions for testing component logic
function validateField(field: keyof ContactMessage, value: string): string | null {
	switch (field) {
		case 'name':
			if (!value.trim()) return 'Le nom est requis';
			if (value.trim().length < 2) return 'Le nom doit contenir au moins 2 caractères';
			return null;
		
		case 'email':
			if (!value.trim()) return 'L\'email est requis';
			const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
			if (!emailRegex.test(value.trim())) return 'Format d\'email invalide';
			return null;
		
		case 'subject':
			if (!value.trim()) return 'Le sujet est requis';
			if (value.trim().length < 5) return 'Le sujet doit contenir au moins 5 caractères';
			return null;
		
		case 'message':
			if (!value.trim()) return 'Le message est requis';
			if (value.trim().length < 10) return 'Le message doit contenir au moins 10 caractères';
			if (value.trim().length > 1000) return 'Le message ne peut pas dépasser 1000 caractères';
			return null;
		
		default:
			return null;
	}
}

function validateForm(formData: ContactMessage): Record<string, string> {
	const errors: Record<string, string> = {};
	
	Object.keys(formData).forEach(key => {
		const field = key as keyof ContactMessage;
		const error = validateField(field, formData[field]);
		if (error) {
			errors[field] = error;
		}
	});
	
	return errors;
}

describe('ContactForm Component Logic', () => {
	it('should have valid contact message data structure', () => {
		expect(mockContactMessage).toHaveProperty('name');
		expect(mockContactMessage).toHaveProperty('email');
		expect(mockContactMessage).toHaveProperty('subject');
		expect(mockContactMessage).toHaveProperty('message');
		
		expect(typeof mockContactMessage.name).toBe('string');
		expect(typeof mockContactMessage.email).toBe('string');
		expect(typeof mockContactMessage.subject).toBe('string');
		expect(typeof mockContactMessage.message).toBe('string');
	});

	describe('Name validation', () => {
		it('should require name', () => {
			expect(validateField('name', '')).toBe('Le nom est requis');
			expect(validateField('name', '   ')).toBe('Le nom est requis');
		});

		it('should require minimum length for name', () => {
			expect(validateField('name', 'A')).toBe('Le nom doit contenir au moins 2 caractères');
			expect(validateField('name', 'AB')).toBeNull();
			expect(validateField('name', 'John Doe')).toBeNull();
		});
	});

	describe('Email validation', () => {
		it('should require email', () => {
			expect(validateField('email', '')).toBe('L\'email est requis');
			expect(validateField('email', '   ')).toBe('L\'email est requis');
		});

		it('should validate email format', () => {
			expect(validateField('email', 'invalid')).toBe('Format d\'email invalide');
			expect(validateField('email', 'invalid@')).toBe('Format d\'email invalide');
			expect(validateField('email', 'invalid@domain')).toBe('Format d\'email invalide');
			expect(validateField('email', '@domain.com')).toBe('Format d\'email invalide');
			
			expect(validateField('email', 'valid@domain.com')).toBeNull();
			expect(validateField('email', 'user.name@example.org')).toBeNull();
			expect(validateField('email', 'test+tag@domain.co.uk')).toBeNull();
		});
	});

	describe('Subject validation', () => {
		it('should require subject', () => {
			expect(validateField('subject', '')).toBe('Le sujet est requis');
			expect(validateField('subject', '   ')).toBe('Le sujet est requis');
		});

		it('should require minimum length for subject', () => {
			expect(validateField('subject', 'Hi')).toBe('Le sujet doit contenir au moins 5 caractères');
			expect(validateField('subject', 'Hello')).toBeNull();
			expect(validateField('subject', 'Demande de collaboration')).toBeNull();
		});
	});

	describe('Message validation', () => {
		it('should require message', () => {
			expect(validateField('message', '')).toBe('Le message est requis');
			expect(validateField('message', '   ')).toBe('Le message est requis');
		});

		it('should require minimum length for message', () => {
			expect(validateField('message', 'Short')).toBe('Le message doit contenir au moins 10 caractères');
			expect(validateField('message', 'This is a valid message')).toBeNull();
		});

		it('should enforce maximum length for message', () => {
			const longMessage = 'A'.repeat(1001);
			const validMessage = 'A'.repeat(1000);
			
			expect(validateField('message', longMessage)).toBe('Le message ne peut pas dépasser 1000 caractères');
			expect(validateField('message', validMessage)).toBeNull();
		});
	});

	describe('Form validation', () => {
		it('should validate complete form successfully', () => {
			const errors = validateForm(mockContactMessage);
			expect(Object.keys(errors)).toHaveLength(0);
		});

		it('should return errors for invalid form', () => {
			const invalidForm: ContactMessage = {
				name: '',
				email: 'invalid-email',
				subject: 'Hi',
				message: 'Short'
			};
			
			const errors = validateForm(invalidForm);
			expect(errors.name).toBe('Le nom est requis');
			expect(errors.email).toBe('Format d\'email invalide');
			expect(errors.subject).toBe('Le sujet doit contenir au moins 5 caractères');
			expect(errors.message).toBe('Le message doit contenir au moins 10 caractères');
		});

		it('should handle partial validation', () => {
			const partialForm: ContactMessage = {
				name: 'John Doe',
				email: 'john@example.com',
				subject: '',
				message: 'This is a valid message with enough characters'
			};
			
			const errors = validateForm(partialForm);
			expect(Object.keys(errors)).toHaveLength(1);
			expect(errors.subject).toBe('Le sujet est requis');
		});
	});

	describe('Edge cases', () => {
		it('should handle whitespace-only inputs', () => {
			expect(validateField('name', '   John   ')).toBeNull(); // Should trim
			expect(validateField('email', '  test@example.com  ')).toBeNull(); // Should trim
		});

		it('should handle special characters in name', () => {
			expect(validateField('name', 'Jean-Pierre')).toBeNull();
			expect(validateField('name', 'Marie O\'Connor')).toBeNull();
			expect(validateField('name', 'José María')).toBeNull();
		});

		it('should handle international email domains', () => {
			expect(validateField('email', 'user@domain.co.uk')).toBeNull();
			expect(validateField('email', 'test@example.org')).toBeNull();
			expect(validateField('email', 'contact@site.museum')).toBeNull();
		});

		it('should handle message at exact limits', () => {
			const message10Chars = 'A'.repeat(10);
			const message1000Chars = 'A'.repeat(1000);
			
			expect(validateField('message', message10Chars)).toBeNull();
			expect(validateField('message', message1000Chars)).toBeNull();
		});
	});

	describe('Character counting', () => {
		it('should count message characters correctly', () => {
			const message = 'Hello world!';
			expect(message.length).toBe(12);
			
			const longMessage = 'A'.repeat(500);
			expect(longMessage.length).toBe(500);
		});

		it('should handle empty message character count', () => {
			expect(''.length).toBe(0);
		});
	});
});