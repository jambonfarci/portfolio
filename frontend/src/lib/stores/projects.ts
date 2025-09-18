import { writable, derived } from 'svelte/store';
import type { Project, CreateProject, UpdateProject, LoadingState } from '$lib/types';
import { apiClient } from '$lib/api/client';
import { toastStore } from './toast';
import { logApiError, logComponentError } from '$lib/utils/errorLogger';

// Store for all projects
export const projects = writable<Project[]>([]);

// Loading state for projects
export const projectsLoading = writable<LoadingState>({
	isLoading: false,
	error: null
});

// Derived store for featured projects
export const featuredProjects = derived(projects, ($projects) =>
	$projects.filter(project => project.featured)
);

// Derived store for project categories
export const projectCategories = derived(projects, ($projects) => {
	const categories = new Set($projects.map(project => project.category));
	return Array.from(categories).sort();
});

// Store actions
export const projectsStore = {
	// Load all projects
	async loadProjects(category?: string, featured?: boolean) {
		projectsLoading.update(state => ({ ...state, isLoading: true, error: null }));
		
		try {
			const response = await apiClient.getProjects(category, featured);
			
			if (response.success && response.data) {
				projects.set(response.data);
				projectsLoading.update(state => ({ ...state, isLoading: false }));
			} else {
				const errorMsg = response.error?.message || 'Erreur lors du chargement des projets';
				logApiError(response.error, 'GET /api/projects');
				projectsLoading.update(state => ({
					...state,
					isLoading: false,
					error: response.error || { code: 'UNKNOWN_ERROR', message: errorMsg }
				}));
				toastStore.error('Erreur de chargement', errorMsg);
			}
		} catch (error) {
			const errorMsg = 'Impossible de se connecter au serveur';
			logApiError(error, 'GET /api/projects');
			projectsLoading.update(state => ({
				...state,
				isLoading: false,
				error: { code: 'NETWORK_ERROR', message: errorMsg }
			}));
			toastStore.error('Erreur réseau', errorMsg);
		}
	},

	// Load a single project
	async loadProject(id: number): Promise<Project | null> {
		try {
			const response = await apiClient.getProject(id);
			
			if (response.success && response.data) {
				return response.data;
			} else {
				console.error('Failed to load project:', response.error);
				return null;
			}
		} catch (error) {
			console.error('Failed to load project:', error);
			return null;
		}
	},

	// Create a new project
	async createProject(projectData: CreateProject): Promise<boolean> {
		try {
			const response = await apiClient.createProject(projectData);
			
			if (response.success && response.data) {
				// Add the new project to the store
				projects.update(currentProjects => [...currentProjects, response.data!]);
				toastStore.success('Projet créé', 'Le projet a été créé avec succès');
				return true;
			} else {
				const errorMsg = response.error?.message || 'Erreur lors de la création du projet';
				toastStore.error('Erreur de création', errorMsg);
				return false;
			}
		} catch (error) {
			toastStore.error('Erreur réseau', 'Impossible de créer le projet');
			return false;
		}
	},

	// Update an existing project
	async updateProject(id: number, projectData: UpdateProject): Promise<boolean> {
		try {
			const response = await apiClient.updateProject(id, projectData);
			
			if (response.success && response.data) {
				// Update the project in the store
				projects.update(currentProjects =>
					currentProjects.map(project =>
						project.id === id ? response.data! : project
					)
				);
				toastStore.success('Projet mis à jour', 'Le projet a été mis à jour avec succès');
				return true;
			} else {
				const errorMsg = response.error?.message || 'Erreur lors de la mise à jour du projet';
				toastStore.error('Erreur de mise à jour', errorMsg);
				return false;
			}
		} catch (error) {
			toastStore.error('Erreur réseau', 'Impossible de mettre à jour le projet');
			return false;
		}
	},

	// Delete a project
	async deleteProject(id: number): Promise<boolean> {
		try {
			const response = await apiClient.deleteProject(id);
			
			if (response.success) {
				// Remove the project from the store
				projects.update(currentProjects =>
					currentProjects.filter(project => project.id !== id)
				);
				toastStore.success('Projet supprimé', 'Le projet a été supprimé avec succès');
				return true;
			} else {
				const errorMsg = response.error?.message || 'Erreur lors de la suppression du projet';
				toastStore.error('Erreur de suppression', errorMsg);
				return false;
			}
		} catch (error) {
			toastStore.error('Erreur réseau', 'Impossible de supprimer le projet');
			return false;
		}
	},

	// Clear error state
	clearError() {
		projectsLoading.update(state => ({ ...state, error: null }));
	}
};