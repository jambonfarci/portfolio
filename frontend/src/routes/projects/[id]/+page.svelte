<script lang="ts">
	import { onMount } from 'svelte';
	import { page } from '$app/stores';
	import { goto } from '$app/navigation';
	import { fade, fly } from 'svelte/transition';
	import { projects, projectsStore } from '$lib/stores/projects';
	import type { Project } from '$lib/types';

	let project: Project | null = null;
	let loading = true;
	let error: string | null = null;
	let pageVisible = false;
	let imageLoaded = false;

	$: projectId = parseInt($page.params.id);

	onMount(async () => {
		try {
			// Load projects if not already loaded
			if ($projects.length === 0) {
				await projectsStore.loadProjects();
			}
			
			// Find the project
			project = $projects.find(p => p.id === projectId) || null;
			
			if (!project) {
				error = 'Projet non trouvé';
			}
		} catch (e) {
			error = 'Erreur lors du chargement du projet';
		} finally {
			loading = false;
			setTimeout(() => pageVisible = true, 100);
		}
	});

	function handleImageLoad() {
		imageLoaded = true;
	}

	function handleImageError() {
		imageLoaded = true;
	}

	// Format date
	function formatDate(dateString: string): string {
		const date = new Date(dateString);
		return date.toLocaleDateString('fr-FR', { 
			year: 'numeric', 
			month: 'long',
			day: 'numeric'
		});
	}

	// Navigate back to projects
	function goBack() {
		goto('/projects');
	}
</script>

<svelte:head>
	<title>{project?.title || 'Projet'} - Portfolio Développeur Web</title>
	<meta name="description" content={project?.description || 'Détails du projet de développement web'} />
</svelte:head>

<div class="min-h-screen bg-white">
	{#if loading}
		<!-- Loading State -->
		<div class="flex items-center justify-center min-h-screen">
			<div class="text-center">
				<div class="animate-spin rounded-full h-12 w-12 border-b-2 border-blue-600 mx-auto mb-4"></div>
				<p class="text-lg text-gray-600">Chargement du projet...</p>
			</div>
		</div>
	{:else if error}
		<!-- Error State -->
		<div class="flex items-center justify-center min-h-screen">
			<div class="text-center max-w-md mx-auto px-4">
				<div class="flex justify-center mb-4">
					<div class="h-16 w-16 rounded-full bg-red-100 flex items-center justify-center">
						<svg class="h-8 w-8 text-red-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
							<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-2.5L13.732 4c-.77-.833-1.964-.833-2.732 0L3.732 16.5c-.77.833.192 2.5 1.732 2.5z" />
						</svg>
					</div>
				</div>
				<h1 class="text-2xl font-bold text-gray-900 mb-2">Projet non trouvé</h1>
				<p class="text-gray-600 mb-6">{error}</p>
				<button
					on:click={goBack}
					class="inline-flex items-center gap-2 rounded-md bg-blue-600 px-4 py-2 text-sm font-semibold text-white shadow-sm hover:bg-blue-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-blue-600 transition-all duration-200"
				>
					<svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
						<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 19l-7-7m0 0l7-7m-7 7h18" />
					</svg>
					Retour aux projets
				</button>
			</div>
		</div>
	{:else if project && pageVisible}
		<!-- Project Detail -->
		<div in:fade={{ duration: 800 }}>
			<!-- Back Navigation -->
			<div class="bg-gray-50 border-b border-gray-200">
				<div class="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8 py-4">
					<button
						on:click={goBack}
						class="inline-flex items-center gap-2 text-sm font-medium text-gray-600 hover:text-gray-900 transition-colors duration-200"
					>
						<svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
							<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 19l-7-7m0 0l7-7m-7 7h18" />
						</svg>
						Retour aux projets
					</button>
				</div>
			</div>

			<!-- Hero Section -->
			<section class="py-16 sm:py-24">
				<div class="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8">
					<div class="lg:grid lg:grid-cols-2 lg:gap-12 lg:items-start">
						<!-- Project Image -->
						<div in:fly={{ x: -50, duration: 600, delay: 200 }}>
							<div class="relative">
								{#if project.image_url}
									<div class="aspect-video overflow-hidden rounded-2xl bg-gray-100 shadow-2xl">
										<img
											src={project.image_url}
											alt={project.title}
											class="h-full w-full object-cover transition-opacity duration-500"
											class:opacity-0={!imageLoaded}
											class:opacity-100={imageLoaded}
											on:load={handleImageLoad}
											on:error={handleImageError}
										/>
									</div>
								{:else}
									<div class="aspect-video flex items-center justify-center bg-gradient-to-br from-blue-50 to-indigo-100 rounded-2xl shadow-2xl">
										<div class="text-center">
											<svg class="mx-auto h-20 w-20 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
												<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10" />
											</svg>
											<p class="mt-2 text-xl text-gray-500">Projet</p>
										</div>
									</div>
								{/if}

								<!-- Featured Badge -->
								{#if project.featured}
									<div class="absolute top-4 left-4">
										<span class="inline-flex items-center gap-1 rounded-full bg-gradient-to-r from-yellow-400 to-orange-500 px-4 py-2 text-sm font-semibold text-white shadow-lg">
											<svg class="w-4 h-4" fill="currentColor" viewBox="0 0 20 20">
												<path d="M9.049 2.927c.3-.921 1.603-.921 1.902 0l1.07 3.292a1 1 0 00.95.69h3.462c.969 0 1.371 1.24.588 1.81l-2.8 2.034a1 1 0 00-.364 1.118l1.07 3.292c.3.921-.755 1.688-1.54 1.118l-2.8-2.034a1 1 0 00-1.175 0l-2.8 2.034c-.784.57-1.838-.197-1.539-1.118l1.07-3.292a1 1 0 00-.364-1.118L2.98 8.72c-.783-.57-.38-1.81.588-1.81h3.461a1 1 0 00.951-.69l1.07-3.292z" />
											</svg>
											Projet vedette
										</span>
									</div>
								{/if}
							</div>
						</div>

						<!-- Project Info -->
						<div in:fly={{ x: 50, duration: 600, delay: 400 }} class="mt-10 lg:mt-0">
							<!-- Header -->
							<div class="mb-8">
								<div class="flex items-start justify-between gap-4 mb-4">
									<h1 class="text-3xl sm:text-4xl lg:text-5xl font-bold text-gray-900">
										{project.title}
									</h1>
									<span class="inline-flex items-center rounded-full bg-gray-100 px-4 py-2 text-sm font-medium text-gray-800 shrink-0">
										{project.category}
									</span>
								</div>
								<p class="text-lg text-gray-600">
									Créé le {formatDate(project.created_at)}
								</p>
							</div>

							<!-- Description -->
							<div class="mb-8">
								<h2 class="text-xl font-semibold text-gray-900 mb-4">Description</h2>
								<p class="text-gray-600 leading-relaxed text-lg">
									{project.description}
								</p>
								{#if project.long_description}
									<div class="mt-6 pt-6 border-t border-gray-200">
										<p class="text-gray-600 leading-relaxed">
											{project.long_description}
										</p>
									</div>
								{/if}
							</div>

							<!-- Technologies -->
							<div class="mb-8">
								<h2 class="text-xl font-semibold text-gray-900 mb-4">Technologies utilisées</h2>
								<div class="flex flex-wrap gap-3">
									{#each project.technologies as tech}
										<span class="inline-flex items-center rounded-lg bg-blue-50 px-4 py-2 text-sm font-medium text-blue-700 ring-1 ring-inset ring-blue-700/10">
											{tech}
										</span>
									{/each}
								</div>
							</div>

							<!-- Actions -->
							<div class="flex flex-wrap gap-4">
								{#if project.demo_url}
									<a
										href={project.demo_url}
										target="_blank"
										rel="noopener noreferrer"
										class="inline-flex items-center gap-2 rounded-lg bg-blue-600 px-6 py-3 text-base font-semibold text-white shadow-sm hover:bg-blue-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-blue-600 transition-all duration-200 transform hover:scale-105"
									>
										<svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
											<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 6H6a2 2 0 00-2 2v10a2 2 0 002 2h10a2 2 0 002-2v-4M14 4h6m0 0v6m0-6L10 14" />
										</svg>
										Voir la démo
									</a>
								{/if}
								
								{#if project.github_url}
									<a
										href={project.github_url}
										target="_blank"
										rel="noopener noreferrer"
										class="inline-flex items-center gap-2 rounded-lg bg-gray-900 px-6 py-3 text-base font-semibold text-white shadow-sm hover:bg-gray-800 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-gray-900 transition-all duration-200 transform hover:scale-105"
									>
										<svg class="w-5 h-5" fill="currentColor" viewBox="0 0 24 24">
											<path d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z"/>
										</svg>
										Voir le code
									</a>
								{/if}
							</div>
						</div>
					</div>
				</div>
			</section>

			<!-- Related Projects or Call to Action -->
			<section class="py-16 bg-gray-50">
				<div class="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8">
					<div class="text-center">
						<h2 class="text-3xl font-bold tracking-tight text-gray-900 sm:text-4xl">
							Découvrez mes autres projets
						</h2>
						<p class="mt-4 text-lg text-gray-600">
							Explorez d'autres réalisations et découvrez la diversité de mon travail
						</p>
						<div class="mt-8 flex items-center justify-center gap-4">
							<button
								on:click={goBack}
								class="rounded-md bg-blue-600 px-6 py-3 text-base font-semibold text-white shadow-sm hover:bg-blue-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-blue-600 transition-all duration-200 transform hover:scale-105"
							>
								Voir tous les projets
							</button>
							<a
								href="/contact"
								class="text-base font-semibold leading-6 text-blue-600 hover:text-blue-500 transition-colors duration-200"
							>
								Me contacter <span aria-hidden="true">→</span>
							</a>
						</div>
					</div>
				</div>
			</section>
		</div>
	{/if}
</div>