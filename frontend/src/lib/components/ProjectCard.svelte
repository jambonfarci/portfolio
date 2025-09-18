<script lang="ts">
	import { fade, fly } from 'svelte/transition';
	import type { Project } from '$lib/types';
	import OptimizedImage from './OptimizedImage.svelte';
	import { lazyLoad } from '$lib/utils/lazyLoad';

	export let project: Project;
	export let featured = false;

	let cardVisible = false;
	let isInView = false;

	// Trigger animation
	setTimeout(() => cardVisible = true, 100);

	function handleLazyLoad() {
		isInView = true;
	}

	// Format date
	function formatDate(dateString: string): string {
		const date = new Date(dateString);
		return date.toLocaleDateString('fr-FR', { 
			year: 'numeric', 
			month: 'long' 
		});
	}

	// Truncate description for card view
	function truncateDescription(text: string, maxLength: number = 120): string {
		if (text.length <= maxLength) return text;
		return text.substring(0, maxLength).trim() + '...';
	}
</script>

{#if cardVisible}
	<article 
		in:fly={{ y: 30, duration: 600 }}
		class="group relative overflow-hidden rounded-2xl bg-white shadow-lg ring-1 ring-gray-200 transition-all duration-300 hover:shadow-xl hover:ring-gray-300 hover:-translate-y-1"
		class:ring-2={featured}
		class:ring-orange-200={featured}
	>
		<!-- Featured Badge -->
		{#if featured}
			<div class="absolute top-4 left-4 z-10">
				<span class="inline-flex items-center gap-1 rounded-full bg-gradient-to-r from-yellow-400 to-orange-500 px-3 py-1 text-xs font-semibold text-white shadow-lg">
					<svg class="w-3 h-3" fill="currentColor" viewBox="0 0 20 20">
						<path d="M9.049 2.927c.3-.921 1.603-.921 1.902 0l1.07 3.292a1 1 0 00.95.69h3.462c.969 0 1.371 1.24.588 1.81l-2.8 2.034a1 1 0 00-.364 1.118l1.07 3.292c.3.921-.755 1.688-1.54 1.118l-2.8-2.034a1 1 0 00-1.175 0l-2.8 2.034c-.784.57-1.838-.197-1.539-1.118l1.07-3.292a1 1 0 00-.364-1.118L2.98 8.72c-.783-.57-.38-1.81.588-1.81h3.461a1 1 0 00.951-.69l1.07-3.292z" />
					</svg>
					Projet vedette
				</span>
			</div>
		{/if}

		<!-- Project Image -->
		<div 
			class="relative aspect-video overflow-hidden bg-gray-100"
			use:lazyLoad
			on:lazyload={handleLazyLoad}
		>
			{#if project.image_url && isInView}
				<OptimizedImage
					src={project.image_url}
					alt={project.title}
					width={400}
					height={225}
					sizes="(max-width: 768px) 100vw, (max-width: 1200px) 50vw, 33vw"
					loading="lazy"
					class="h-full w-full transition-all duration-500 group-hover:scale-105"
					placeholder="blur"
				/>
			{:else if !project.image_url}
				<div class="flex h-full w-full items-center justify-center bg-gradient-to-br from-blue-50 to-indigo-100">
					<div class="text-center">
						<svg class="mx-auto h-12 w-12 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
							<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10" />
						</svg>
						<p class="mt-2 text-sm text-gray-500">Projet</p>
					</div>
				</div>
			{/if}
			
			<!-- Overlay gradient -->
			<div class="absolute inset-0 bg-gradient-to-t from-black/20 to-transparent opacity-0 group-hover:opacity-100 transition-opacity duration-300"></div>
		</div>

		<!-- Project Content -->
		<div class="p-6">
			<!-- Title and Date -->
			<div class="mb-3">
				<h3 class="text-xl font-bold text-gray-900 group-hover:text-blue-600 transition-colors duration-200">
					{project.title}
				</h3>
				<p class="text-sm text-gray-500 mt-1">
					{formatDate(project.created_at)}
				</p>
			</div>

			<!-- Description -->
			<p class="text-gray-600 text-sm leading-relaxed mb-4">
				{truncateDescription(project.description)}
			</p>

			<!-- Technologies -->
			<div class="mb-4">
				<div class="flex flex-wrap gap-2">
					{#each project.technologies.slice(0, 4) as tech}
						<span class="inline-flex items-center rounded-md bg-blue-50 px-2 py-1 text-xs font-medium text-blue-700 ring-1 ring-inset ring-blue-700/10">
							{tech}
						</span>
					{/each}
					{#if project.technologies.length > 4}
						<span class="inline-flex items-center rounded-md bg-gray-50 px-2 py-1 text-xs font-medium text-gray-600">
							+{project.technologies.length - 4} autres
						</span>
					{/if}
				</div>
			</div>

			<!-- Category -->
			<div class="mb-4">
				<span class="inline-flex items-center rounded-full bg-gray-100 px-3 py-1 text-xs font-medium text-gray-800">
					{project.category}
				</span>
			</div>

			<!-- Actions -->
			<div class="flex items-center gap-3">
				{#if project.demo_url}
					<a
						href={project.demo_url}
						target="_blank"
						rel="noopener noreferrer"
						class="inline-flex items-center gap-2 rounded-md bg-blue-600 px-3 py-2 text-sm font-semibold text-white shadow-sm hover:bg-blue-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-blue-600 transition-all duration-200 transform hover:scale-105"
					>
						<svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
							<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 6H6a2 2 0 00-2 2v10a2 2 0 002 2h10a2 2 0 002-2v-4M14 4h6m0 0v6m0-6L10 14" />
						</svg>
						Démo
					</a>
				{/if}
				
				{#if project.github_url}
					<a
						href={project.github_url}
						target="_blank"
						rel="noopener noreferrer"
						class="inline-flex items-center gap-2 rounded-md bg-gray-900 px-3 py-2 text-sm font-semibold text-white shadow-sm hover:bg-gray-800 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-gray-900 transition-all duration-200 transform hover:scale-105"
					>
						<svg class="w-4 h-4" fill="currentColor" viewBox="0 0 24 24">
							<path d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z"/>
						</svg>
						Code
					</a>
				{/if}

				<!-- View Details Button -->
				<a
					href="/projects/{project.id}"
					class="ml-auto inline-flex items-center gap-1 text-sm font-medium text-blue-600 hover:text-blue-500 transition-colors duration-200"
				>
					Détails
					<svg class="w-4 h-4 transition-transform duration-200 group-hover:translate-x-1" fill="none" stroke="currentColor" viewBox="0 0 24 24">
						<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 8l4 4m0 0l-4 4m4-4H3" />
					</svg>
				</a>
			</div>
		</div>

		<!-- Hover effect overlay -->
		<div class="absolute inset-0 rounded-2xl bg-gradient-to-r from-blue-600/5 to-purple-600/5 opacity-0 group-hover:opacity-100 transition-opacity duration-300 pointer-events-none"></div>
	</article>
{/if}

