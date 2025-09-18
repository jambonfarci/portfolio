<script lang="ts">
	import { onMount } from 'svelte';
	import { fade, fly } from 'svelte/transition';
	import { projects, projectCategories, projectsStore, projectsLoading } from '$lib/stores/projects';
	import ProjectGrid from '$lib/components/ProjectGrid.svelte';
	import ProjectModal from '$lib/components/ProjectModal.svelte';
	import type { Project } from '$lib/types';

	let pageVisible = false;
	let selectedProject: Project | null = null;
	let isModalOpen = false;

	onMount(async () => {
		// Load projects data
		await projectsStore.loadProjects();
		
		// Trigger page animation
		setTimeout(() => pageVisible = true, 100);
	});

	function openProjectModal(project: Project) {
		selectedProject = project;
		isModalOpen = true;
	}

	function closeProjectModal() {
		isModalOpen = false;
		selectedProject = null;
	}

	// Handle project card clicks (if we want to open modal instead of navigating)
	function handleProjectClick(event: CustomEvent<Project>) {
		openProjectModal(event.detail);
	}
</script>

<svelte:head>
	<title>Projets - Portfolio Développeur Web</title>
	<meta name="description" content="Découvrez mes projets de développement web utilisant Rust, Svelte et d'autres technologies modernes." />
</svelte:head>

<div class="min-h-screen bg-white">
	<!-- Hero Section -->
	{#if pageVisible}
		<section in:fade={{ duration: 800 }} class="relative bg-gradient-to-br from-blue-50 to-indigo-100 py-16 sm:py-24">
			<div class="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8">
				<div in:fly={{ y: 30, duration: 600, delay: 200 }} class="text-center">
					<h1 class="text-4xl font-bold tracking-tight text-gray-900 sm:text-5xl lg:text-6xl">
						Mes Projets
					</h1>
					<p class="mt-6 text-lg leading-8 text-gray-600 max-w-2xl mx-auto">
						Découvrez une sélection de mes réalisations en développement web, 
						allant des applications full-stack aux outils spécialisés.
					</p>
				</div>
			</div>
			
			<!-- Decorative elements -->
			<div class="absolute top-0 left-0 w-full h-full overflow-hidden pointer-events-none">
				<div class="absolute top-1/4 left-1/4 w-32 h-32 bg-blue-200 opacity-20 rounded-full blur-xl"></div>
				<div class="absolute bottom-1/4 right-1/4 w-48 h-48 bg-indigo-200 opacity-20 rounded-full blur-xl"></div>
			</div>
		</section>
	{/if}

	<!-- Projects Section -->
	<section class="py-16 sm:py-24">
		<div class="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8">
			{#if $projectsLoading.isLoading}
				<!-- Loading State -->
				<div class="text-center py-12">
					<div class="inline-flex items-center gap-3">
						<div class="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-600"></div>
						<span class="text-lg text-gray-600">Chargement des projets...</span>
					</div>
				</div>
			{:else if $projectsLoading.error}
				<!-- Error State -->
				<div class="text-center py-12">
					<div class="mx-auto max-w-md">
						<div class="flex justify-center mb-4">
							<div class="h-16 w-16 rounded-full bg-red-100 flex items-center justify-center">
								<svg class="h-8 w-8 text-red-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
									<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-2.5L13.732 4c-.77-.833-1.964-.833-2.732 0L3.732 16.5c-.77.833.192 2.5 1.732 2.5z" />
								</svg>
							</div>
						</div>
						<h3 class="text-lg font-medium text-gray-900 mb-2">Erreur de chargement</h3>
						<p class="text-gray-500 mb-4">
							{$projectsLoading.error.message}
						</p>
						<button
							on:click={() => projectsStore.loadProjects()}
							class="inline-flex items-center gap-2 rounded-md bg-blue-600 px-4 py-2 text-sm font-semibold text-white shadow-sm hover:bg-blue-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-blue-600 transition-all duration-200"
						>
							<svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
								<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
							</svg>
							Réessayer
						</button>
					</div>
				</div>
			{:else}
				<!-- Projects Grid -->
				{#if pageVisible}
					<div in:fly={{ y: 30, duration: 600, delay: 400 }}>
						<ProjectGrid 
							projects={$projects} 
							categories={$projectCategories}
							showFilters={true}
							showFeaturedFirst={true}
						/>
					</div>
				{/if}
			{/if}
		</div>
	</section>

	<!-- Call to Action -->
	{#if pageVisible && $projects.length > 0}
		<section in:fly={{ y: 30, duration: 600, delay: 600 }} class="py-16 bg-gray-50">
			<div class="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8">
				<div class="text-center">
					<h2 class="text-3xl font-bold tracking-tight text-gray-900 sm:text-4xl">
						Intéressé par mon travail ?
					</h2>
					<p class="mt-4 text-lg text-gray-600">
						N'hésitez pas à me contacter pour discuter de votre prochain projet
					</p>
					<div class="mt-8 flex items-center justify-center gap-4">
						<a
							href="/contact"
							class="rounded-md bg-blue-600 px-6 py-3 text-base font-semibold text-white shadow-sm hover:bg-blue-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-blue-600 transition-all duration-200 transform hover:scale-105"
						>
							Me contacter
						</a>
						<a
							href="/"
							class="text-base font-semibold leading-6 text-blue-600 hover:text-blue-500 transition-colors duration-200"
						>
							Retour à l'accueil <span aria-hidden="true">→</span>
						</a>
					</div>
				</div>
			</div>
		</section>
	{/if}
</div>

<!-- Project Modal -->
<ProjectModal 
	{selectedProject} 
	{isModalOpen} 
	on:close={closeProjectModal} 
/>