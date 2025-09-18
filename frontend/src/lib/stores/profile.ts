import { writable } from 'svelte/store';
import type { Profile, UpdateProfile, LoadingState } from '$lib/types';
import { apiClient } from '$lib/api/client';

// Store for profile data
export const profile = writable<Profile | null>(null);

// Loading state for profile
export const profileLoading = writable<LoadingState>({
	isLoading: false,
	error: null
});

// Store actions
export const profileStore = {
	// Load profile data
	async loadProfile() {
		profileLoading.update(state => ({ ...state, isLoading: true, error: null }));
		
		try {
			const response = await apiClient.getProfile();
			
			if (response.success && response.data) {
				profile.set(response.data);
				profileLoading.update(state => ({ ...state, isLoading: false }));
			} else {
				profileLoading.update(state => ({
					...state,
					isLoading: false,
					error: response.error || { code: 'UNKNOWN_ERROR', message: 'Unknown error occurred' }
				}));
			}
		} catch (error) {
			profileLoading.update(state => ({
				...state,
				isLoading: false,
				error: { code: 'NETWORK_ERROR', message: 'Failed to load profile' }
			}));
		}
	},

	// Update profile data
	async updateProfile(profileData: UpdateProfile): Promise<boolean> {
		try {
			const response = await apiClient.updateProfile(profileData);
			
			if (response.success && response.data) {
				// Update the profile in the store
				profile.set(response.data);
				return true;
			} else {
				console.error('Failed to update profile:', response.error);
				return false;
			}
		} catch (error) {
			console.error('Failed to update profile:', error);
			return false;
		}
	},

	// Clear error state
	clearError() {
		profileLoading.update(state => ({ ...state, error: null }));
	}
};