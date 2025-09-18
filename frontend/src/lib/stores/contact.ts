import { writable } from 'svelte/store';
import type { ContactMessage, LoadingState } from '$lib/types';
import { apiClient } from '$lib/api/client';
import { toastStore } from './toast';

// Loading state for contact form
export const contactLoading = writable<LoadingState>({
	isLoading: false,
	error: null
});

// Success state for contact form
export const contactSuccess = writable<boolean>(false);

// Store actions
export const contactStore = {
	// Send contact message
	async sendMessage(message: ContactMessage): Promise<boolean> {
		contactLoading.update(state => ({ ...state, isLoading: true, error: null }));
		contactSuccess.set(false);
		
		try {
			const response = await apiClient.sendContactMessage(message);
			
			if (response.success) {
				contactLoading.update(state => ({ ...state, isLoading: false }));
				contactSuccess.set(true);
				toastStore.success('Message envoyé', 'Votre message a été envoyé avec succès. Je vous répondrai bientôt !');
				return true;
			} else {
				const errorMsg = response.error?.message || 'Erreur lors de l\'envoi du message';
				contactLoading.update(state => ({
					...state,
					isLoading: false,
					error: response.error || { code: 'UNKNOWN_ERROR', message: errorMsg }
				}));
				toastStore.error('Erreur d\'envoi', errorMsg);
				return false;
			}
		} catch (error) {
			const errorMsg = 'Impossible de se connecter au serveur';
			contactLoading.update(state => ({
				...state,
				isLoading: false,
				error: { code: 'NETWORK_ERROR', message: errorMsg }
			}));
			toastStore.error('Erreur réseau', errorMsg);
			return false;
		}
	},

	// Clear states
	clearError() {
		contactLoading.update(state => ({ ...state, error: null }));
	},

	clearSuccess() {
		contactSuccess.set(false);
	},

	reset() {
		contactLoading.set({ isLoading: false, error: null });
		contactSuccess.set(false);
	}
};