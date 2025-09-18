<script lang="ts">
	import { fade, fly, scale } from 'svelte/transition';
	import { createEventDispatcher } from 'svelte';
	import type { Project } from '$lib/types';

	export let project: Project | null = null;
	export let isOpen = false;

	const dispatch = createEventDispatcher();

	let modalElement: HTMLElement;
	let imageLoaded = false;

	function closeModal() {
		isOpen = false;
		dispatch('close');
	}

	function handleKeydown(event: KeyboardEvent) {
		if (event.key === 'Escape') {
			closeModal();
		}
	}

	function handleBackdropClick(event: MouseEvent) {
		if (event.target === modalElement) {
			closeModal();
		}
	}

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



	// Prevent body scroll when modal is open
	$: if (typeof document !== 'undefined') {
		if (isOpen) {
			document.body.style.overflow = 'hidden';
		} else {
			document.body.style.overflow = '';
		}
	}
</script>

<svelte:window on:keydown={handleKeydown} />

{#if isOpen && project}
	<!-- Modal Backdrop -->
	<div
		bind:this={modalElement}
		in:fade={{ duration: 300 }}
		out:fade={{ duration: 200 }}
		on:click={handleBackdropClick}
		on:keydown={handleKeydown}
		class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black bg-opacity-50 backdrop-blur-sm"
		role="dialog"
		aria-modal="true"
		tabindex="-1"
		aria-labelledby="modal-title"
	>
		<!-- Modal Content -->
		<div
			in:scale={{ duration: 300, start: 0.95 }}
			out:scale={{ duration: 200, start: 0.95 }}
			class="relative w-full max-w-4xl max-h-[90vh] overflow-hidden rounded-2xl bg-white shadow-2xl"
		>
			<!-- Close Button -->
			<button
				on:click={closeModal}
				class="absolute top-4 right-4 z-10 flex h-10 w-10 items-center justify-center rounded-full bg-white bg-opacity-90 text-gray-600 shadow-lg backdrop-blur-sm transition-all duration-200 hover:bg-white hover:text-gray-900 focus:outline-none focus:ring-2 focus:ring-blue-500"
				aria-label="Fermer le modal"
			>
				<svg class="h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
					<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
				</svg>
			</button>

			<!-- Modal Body -->
			<div class="flex flex-col lg:flex-row max-h-[90vh]">
				<!-- Project Image -->
				<div class="lg:w-1/2 relative">
					{#if project.image_url}
						<div class="aspect-video lg:aspect-square relative overflow-hidden bg-gray-100">
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
						<div class="aspect-video lg:aspect-square flex items-center justify-center bg-gradient-to-br from-blue-50 to-indigo-100">
							<div class="text-center">
								<svg class="mx-auto h-16 w-16 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
									<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10" />
								</svg>
								<p class="mt-2 text-lg text-gray-500">Projet</p>
							</div>
						</div>
					{/if}

					<!-- Featured Badge -->
					{#if project.featured}
						<div class="absolute top-4 left-4">
							<span class="inline-flex items-center gap-1 rounded-full bg-gradient-to-r from-yellow-400 to-orange-500 px-3 py-1 text-sm font-semibold text-white shadow-lg">
								<svg class="w-4 h-4" fill="currentColor" viewBox="0 0 20 20">
									<path d="M9.049 2.927c.3-.921 1.603-.921 1.902 0l1.07 3.292a1 1 0 00.95.69h3.462c.969 0 1.371 1.24.588 1.81l-2.8 2.034a1 1 0 00-.364 1.118l1.07 3.292c.3.921-.755 1.688-1.54 1.118l-2.8-2.034a1 1 0 00-1.175 0l-2.8 2.034c-.784.57-1.838-.197-1.539-1.118l1.07-3.292a1 1 0 00-.364-1.118L2.98 8.72c-.783-.57-.38-1.81.588-1.81h3.461a1 1 0 00.951-.69l1.07-3.292z" />
								</svg>
								Projet vedette
							</span>
						</div>
					{/if}
				</div>

				<!-- Project Details -->
				<div class="lg:w-1/2 flex flex-col">
					<div class="flex-1 overflow-y-auto p-6 lg:p-8">
						<!-- Header -->
						<div class="mb-6">
							<div class="flex items-start justify-between gap-4 mb-2">
								<h2 id="modal-title" class="text-2xl lg:text-3xl font-bold text-gray-900">
									{project.title}
								</h2>
								<span class="inline-flex items-center rounded-full bg-gray-100 px-3 py-1 text-sm font-medium text-gray-800 shrink-0">
									{project.category}
								</span>
							</div>
							<p class="text-sm text-gray-500">
								Créé le {formatDate(project.created_at)}
							</p>
						</div>

						<!-- Description -->
						<div class="mb-6">
							<h3 class="text-lg font-semibold text-gray-900 mb-3">Description</h3>
							<p class="text-gray-600 leading-relaxed">
								{project.description}
							</p>
							{#if project.long_description}
								<div class="mt-4 pt-4 border-t border-gray-100">
									<p class="text-gray-600 leading-relaxed">
										{project.long_description}
									</p>
								</div>
							{/if}
						</div>

						<!-- Technologies -->
						<div class="mb-6">
							<h3 class="text-lg font-semibold text-gray-900 mb-3">Technologies utilisées</h3>
							<div class="flex flex-wrap gap-2">
								{#each project.technologies as tech}
									<span class="inline-flex items-center rounded-lg bg-blue-50 px-3 py-2 text-sm font-medium text-blue-700 ring-1 ring-inset ring-blue-700/10">
										{tech}
									</span>
								{/each}
							</div>
						</div>

						<!-- Links -->
						<div class="mb-6">
							<h3 class="text-lg font-semibold text-gray-900 mb-3">Liens</h3>
							<div class="flex flex-wrap gap-3">
								{#if project.demo_url}
									<a
										href={project.demo_url}
										target="_blank"
										rel="noopener noreferrer"
										class="inline-flex items-center gap-2 rounded-lg bg-blue-600 px-4 py-2 text-sm font-semibold text-white shadow-sm hover:bg-blue-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-blue-600 transition-all duration-200 transform hover:scale-105"
									>
										<svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
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
										class="inline-flex items-center gap-2 rounded-lg bg-gray-900 px-4 py-2 text-sm font-semibold text-white shadow-sm hover:bg-gray-800 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-gray-900 transition-all duration-200 transform hover:scale-105"
									>
										<svg class="w-4 h-4" fill="currentColor" viewBox="0 0 24 24">
											<path d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z"/>
										</svg>
										Voir le code
									</a>
								{/if}
							</div>
						</div>
					</div>

					<!-- Modal Footer -->
					<div class="border-t border-gray-200 px-6 py-4 lg:px-8">
						<div class="flex items-center justify-between">
							<p class="text-sm text-gray-500">
								Projet créé le {formatDate(project.created_at)}
							</p>
							<button
								on:click={closeModal}
								class="rounded-lg bg-gray-100 px-4 py-2 text-sm font-medium text-gray-700 hover:bg-gray-200 focus:outline-none focus:ring-2 focus:ring-gray-500 focus:ring-offset-2 transition-colors duration-200"
							>
								Fermer
							</button>
						</div>
					</div>
				</div>
			</div>
		</div>
	</div>
{/if}