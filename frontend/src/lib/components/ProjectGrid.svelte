<script lang="ts">
	import { fade, fly } from 'svelte/transition';
	import type { Project } from '$lib/types';
	import ProjectCard from './ProjectCard.svelte';
	import { stagger, slideFade, reveal } from '$lib/utils/animations';
	import { createScreenSizeStore, getGridColumns, isMobile, isTouchDevice } from '$lib/utils/responsive';
	import { onMount } from 'svelte';

	export let projects: Project[] = [];
	export let categories: string[] = [];
	export let showFilters = true;
	export let showFeaturedFirst = true;

	let selectedCategory = 'all';
	let searchQuery = '';
	let gridVisible = false;
	let filtersExpanded = false;
	let gridContainer: HTMLElement;

	// Responsive store
	const screenSize = createScreenSizeStore();

	// Trigger animation
	onMount(() => {
		setTimeout(() => gridVisible = true, 200);
	});

	// Auto-collapse filters on mobile when not in use
	$: if ($screenSize.isMobile && searchQuery === '' && selectedCategory === 'all') {
		filtersExpanded = false;
	}

	// Filter projects based on category and search
	$: filteredProjects = projects.filter(project => {
		const matchesCategory = selectedCategory === 'all' || project.category === selectedCategory;
		const matchesSearch = searchQuery === '' || 
			project.title.toLowerCase().includes(searchQuery.toLowerCase()) ||
			project.description.toLowerCase().includes(searchQuery.toLowerCase()) ||
			project.technologies.some(tech => tech.toLowerCase().includes(searchQuery.toLowerCase()));
		
		return matchesCategory && matchesSearch;
	});

	// Sort projects (featured first if enabled)
	$: sortedProjects = showFeaturedFirst 
		? [...filteredProjects].sort((a, b) => {
			if (a.featured && !b.featured) return -1;
			if (!a.featured && b.featured) return 1;
			return new Date(b.created_at).getTime() - new Date(a.created_at).getTime();
		})
		: [...filteredProjects].sort((a, b) => 
			new Date(b.created_at).getTime() - new Date(a.created_at).getTime()
		);

	// Get project count for each category
	function getCategoryCount(category: string): number {
		if (category === 'all') return projects.length;
		return projects.filter(p => p.category === category).length;
	}

	// Clear filters
	function clearFilters() {
		selectedCategory = 'all';
		searchQuery = '';
	}
</script>

<div class="w-full">
	{#if showFilters && gridVisible}
		<!-- Filters Section -->
		<div in:slideFade={{ direction: 'down', duration: 600 }} class="mb-6 sm:mb-8">
			<!-- Mobile Filter Toggle -->
			{#if $screenSize.isMobile}
				<button
					on:click={() => filtersExpanded = !filtersExpanded}
					class="w-full flex items-center justify-between p-4 bg-gray-50 rounded-lg mb-4 transition-all duration-200"
					class:bg-blue-50={filtersExpanded}
					class:border-blue-200={filtersExpanded}
				>
					<div class="flex items-center gap-2">
						<svg class="w-5 h-5 text-gray-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
							<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 4a1 1 0 011-1h16a1 1 0 011 1v2.586a1 1 0 01-.293.707l-6.414 6.414a1 1 0 00-.293.707V17l-4 4v-6.586a1 1 0 00-.293-.707L3.293 7.414A1 1 0 013 6.707V4z" />
						</svg>
						<span class="font-medium text-gray-900">Filtres</span>
						{#if selectedCategory !== 'all' || searchQuery}
							<span class="inline-flex items-center justify-center w-5 h-5 text-xs bg-blue-600 text-white rounded-full">
								{(selectedCategory !== 'all' ? 1 : 0) + (searchQuery ? 1 : 0)}
							</span>
						{/if}
					</div>
					<svg 
						class="w-5 h-5 text-gray-400 transition-transform duration-200"
						class:rotate-180={filtersExpanded}
						fill="none" 
						stroke="currentColor" 
						viewBox="0 0 24 24"
					>
						<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7" />
					</svg>
				</button>
			{/if}

			<div 
				class="overflow-hidden transition-all duration-300"
				class:max-h-0={$screenSize.isMobile && !filtersExpanded}
				class:max-h-96={$screenSize.isMobile && filtersExpanded}
				class:max-h-none={!$screenSize.isMobile}
			>
				<div class="flex flex-col gap-4 sm:flex-row sm:items-center sm:justify-between">
				<!-- Search -->
				<div class="relative flex-1 max-w-md">
					<div class="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
						<svg class="h-5 w-5 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
							<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
						</svg>
					</div>
					<input
						type="text"
						bind:value={searchQuery}
						placeholder="Rechercher un projet..."
						class="block w-full pl-10 pr-3 py-2 border border-gray-300 rounded-md leading-5 bg-white placeholder-gray-500 focus:outline-none focus:placeholder-gray-400 focus:ring-1 focus:ring-blue-500 focus:border-blue-500 text-sm"
					/>
					{#if searchQuery}
						<button
							on:click={() => searchQuery = ''}
							class="absolute inset-y-0 right-0 pr-3 flex items-center"
							aria-label="Effacer la recherche"
						>
							<svg class="h-4 w-4 text-gray-400 hover:text-gray-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
								<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
							</svg>
						</button>
					{/if}
				</div>

				<!-- Category Filters -->
				<div class="flex flex-wrap gap-2">
					<button
						on:click={() => selectedCategory = 'all'}
						class="inline-flex items-center gap-2 rounded-full px-4 py-2 text-sm font-medium transition-all duration-200"
						class:bg-blue-600={selectedCategory === 'all'}
						class:text-white={selectedCategory === 'all'}
						class:bg-gray-100={selectedCategory !== 'all'}
						class:text-gray-700={selectedCategory !== 'all'}
						class:hover:bg-blue-500={selectedCategory === 'all'}
						class:hover:bg-gray-200={selectedCategory !== 'all'}
					>
						Tous
						<span class="inline-flex items-center justify-center w-5 h-5 text-xs rounded-full"
							class:bg-white={selectedCategory === 'all'}
							class:bg-opacity-20={selectedCategory === 'all'}
							class:text-white={selectedCategory === 'all'}
							class:bg-blue-100={selectedCategory !== 'all'}
							class:text-blue-600={selectedCategory !== 'all'}
						>
							{getCategoryCount('all')}
						</span>
					</button>
					
					{#each categories as category}
						<button
							on:click={() => selectedCategory = category}
							class="inline-flex items-center gap-2 rounded-full px-4 py-2 text-sm font-medium transition-all duration-200"
							class:bg-blue-600={selectedCategory === category}
							class:text-white={selectedCategory === category}
							class:bg-gray-100={selectedCategory !== category}
							class:text-gray-700={selectedCategory !== category}
							class:hover:bg-blue-500={selectedCategory === category}
							class:hover:bg-gray-200={selectedCategory !== category}
						>
							{category}
							<span class="inline-flex items-center justify-center w-5 h-5 text-xs rounded-full"
								class:bg-white={selectedCategory === category}
								class:bg-opacity-20={selectedCategory === category}
								class:text-white={selectedCategory === category}
								class:bg-blue-100={selectedCategory !== category}
								class:text-blue-600={selectedCategory !== category}
							>
								{getCategoryCount(category)}
							</span>
						</button>
					{/each}
				</div>
			</div>
		</div>

		<!-- Active Filters & Clear -->
			{#if selectedCategory !== 'all' || searchQuery}
				<div class="mt-4 flex items-center gap-2">
					<span class="text-sm text-gray-500">Filtres actifs:</span>
					{#if selectedCategory !== 'all'}
						<span class="inline-flex items-center gap-1 rounded-md bg-blue-50 px-2 py-1 text-xs font-medium text-blue-700">
							{selectedCategory}
							<button on:click={() => selectedCategory = 'all'} class="hover:text-blue-900" aria-label="Supprimer le filtre de catégorie">
								<svg class="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
									<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
								</svg>
							</button>
						</span>
					{/if}
					{#if searchQuery}
						<span class="inline-flex items-center gap-1 rounded-md bg-green-50 px-2 py-1 text-xs font-medium text-green-700">
							"{searchQuery}"
							<button on:click={() => searchQuery = ''} class="hover:text-green-900" aria-label="Effacer la recherche">
								<svg class="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
									<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
								</svg>
							</button>
						</span>
					{/if}
					<button
						on:click={clearFilters}
						class="text-xs text-gray-500 hover:text-gray-700 underline"
					>
						Effacer tous les filtres
					</button>
				</div>
			{/if}
		</div>
	{/if}

	<!-- Results Count -->
	{#if gridVisible}
		<div in:fade={{ duration: 400, delay: 200 }} class="mb-6">
			<p class="text-sm text-gray-600">
				{#if filteredProjects.length === 0}
					Aucun projet trouvé
				{:else if filteredProjects.length === 1}
					1 projet trouvé
				{:else}
					{filteredProjects.length} projets trouvés
				{/if}
				{#if selectedCategory !== 'all' || searchQuery}
					sur {projects.length} au total
				{/if}
			</p>
		</div>
	{/if}

	<!-- Projects Grid -->
	{#if sortedProjects.length > 0}
		<div 
			bind:this={gridContainer}
			class="grid gap-4 sm:gap-6"
			class:grid-cols-1={$screenSize.isMobile}
			class:grid-cols-2={$screenSize.isTablet || ($screenSize.isDesktop && sortedProjects.length < 3)}
			class:grid-cols-3={$screenSize.isDesktop && sortedProjects.length >= 3}
			class:lg:grid-cols-4={sortedProjects.length >= 4}
			use:reveal
			on:reveal={() => gridVisible = true}
		>
			{#each sortedProjects as project, index}
				<div 
					in:stagger={{ delay: 100, staggerDelay: 50 }}
					class="transform transition-all duration-300 hover:scale-105"
					class:hover:z-10={!$screenSize.isMobile}
				>
					<ProjectCard {project} featured={project.featured} />
				</div>
			{/each}
		</div>
	{:else if gridVisible}
		<!-- Empty State -->
		<div in:fade={{ duration: 600, delay: 300 }} class="text-center py-12">
			<div class="mx-auto max-w-md">
				<div class="flex justify-center mb-4">
					<div class="h-16 w-16 rounded-full bg-gray-100 flex items-center justify-center">
						<svg class="h-8 w-8 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
							<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10" />
						</svg>
					</div>
				</div>
				<h3 class="text-lg font-medium text-gray-900 mb-2">Aucun projet trouvé</h3>
				<p class="text-gray-500 mb-4">
					{#if searchQuery && selectedCategory !== 'all'}
						Aucun projet ne correspond à votre recherche "{searchQuery}" dans la catégorie "{selectedCategory}".
					{:else if searchQuery}
						Aucun projet ne correspond à votre recherche "{searchQuery}".
					{:else if selectedCategory !== 'all'}
						Aucun projet trouvé dans la catégorie "{selectedCategory}".
					{:else}
						Aucun projet disponible pour le moment.
					{/if}
				</p>
				{#if selectedCategory !== 'all' || searchQuery}
					<button
						on:click={clearFilters}
						class="inline-flex items-center gap-2 rounded-md bg-blue-600 px-4 py-2 text-sm font-semibold text-white shadow-sm hover:bg-blue-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-blue-600 transition-all duration-200"
					>
						<svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
							<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
						</svg>
						Voir tous les projets
					</button>
				{/if}
			</div>
		</div>
	{/if}
</div>