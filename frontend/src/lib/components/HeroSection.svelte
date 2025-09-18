<script lang="ts">
	import { fly, fade } from 'svelte/transition';
	import type { Profile } from '$lib/types';
	import OptimizedImage from './OptimizedImage.svelte';
	import { slideIn, fadeScale, parallax } from '$lib/utils/animations';
	import { createScreenSizeStore, getResponsiveTextSize } from '$lib/utils/responsive';
	import { onMount } from 'svelte';

	export let profile: Profile | null;

	let contentVisible = false;
	let heroSection: HTMLElement;

	// Responsive store
	const screenSize = createScreenSizeStore();

	// Trigger content animation after component mounts
	onMount(() => {
		setTimeout(() => contentVisible = true, 200);
	});
</script>

<section 
	bind:this={heroSection}
	class="relative bg-gradient-to-br from-blue-600 via-purple-600 to-indigo-700 text-white overflow-hidden min-h-screen flex items-center"
	class:min-h-[80vh]={$screenSize.isMobile}
>
	<!-- Background decoration -->
	<div class="absolute inset-0 bg-black opacity-10"></div>
	<div 
		class="absolute inset-0"
		use:parallax={{ speed: 0.3 }}
	>
		<div class="absolute top-0 left-0 w-full h-full">
			<div class="absolute top-1/4 left-1/4 w-32 h-32 sm:w-64 sm:h-64 bg-white opacity-5 rounded-full blur-3xl"></div>
			<div class="absolute bottom-1/4 right-1/4 w-48 h-48 sm:w-96 sm:h-96 bg-purple-300 opacity-5 rounded-full blur-3xl"></div>
		</div>
	</div>

	<div class="relative mx-auto max-w-7xl px-4 py-12 sm:px-6 lg:px-8 sm:py-16 lg:py-24">
		<div class="flex flex-col lg:grid lg:grid-cols-12 lg:gap-8 items-center">
			<!-- Profile Image -->
			<div 
				class="lg:col-span-5 mb-8 lg:mb-0"
				class:order-2={$screenSize.isMobile}
				class:lg:order-1={true}
			>
				{#if contentVisible}
					<div 
						in:slideIn={{ direction: $screenSize.isMobile ? 'up' : 'left', duration: 800, delay: 300 }} 
						class="flex justify-center lg:justify-start"
					>
						<div class="relative">
							<div 
								class="rounded-full overflow-hidden shadow-2xl ring-4 ring-white/20 transition-all duration-300 hover:ring-8 hover:ring-white/30"
								class:w-48={$screenSize.isMobile}
								class:h-48={$screenSize.isMobile}
								class:w-64={$screenSize.isTablet}
								class:h-64={$screenSize.isTablet}
								class:sm:w-80={$screenSize.isDesktop}
								class:sm:h-80={$screenSize.isDesktop}
							>
								<div class="w-full h-full bg-gradient-to-br from-blue-400 to-purple-500 flex items-center justify-center">
									<span class="text-6xl font-bold text-white">
										{profile?.name?.charAt(0) || 'D'}
									</span>
								</div>
							</div>
							<!-- Decorative ring -->
							<div class="absolute -inset-4 rounded-full border-2 border-white/10 animate-pulse"></div>
						</div>
					</div>
				{/if}
			</div>

			<!-- Hero Content -->
			<div 
				class="lg:col-span-7 text-center lg:text-left"
				class:order-1={$screenSize.isMobile}
				class:lg:order-2={true}
			>
				{#if contentVisible}
					<div in:slideIn={{ direction: $screenSize.isMobile ? 'up' : 'right', duration: 800, delay: 100 }}>
						<h1 
							class="font-bold tracking-tight"
							class:text-3xl={$screenSize.isMobile}
							class:text-4xl={$screenSize.isTablet}
							class:sm:text-5xl={$screenSize.isDesktop}
							class:lg:text-6xl={$screenSize.isDesktop}
						>
							{#if profile?.name}
								<span class="block">{profile.name}</span>
							{:else}
								<span class="block">Développeur Web</span>
							{/if}
						</h1>
						
						<div in:fadeScale={{ duration: 600, delay: 500 }}>
							<p 
								class="mt-4 font-medium text-blue-100"
								class:text-lg={$screenSize.isMobile}
								class:text-xl={$screenSize.isTablet}
								class:sm:text-2xl={$screenSize.isDesktop}
							>
								{profile?.title || 'Spécialisé en Rust et Svelte'}
							</p>
						</div>

						<div in:fadeScale={{ duration: 600, delay: 700 }}>
							<p 
								class="mt-4 sm:mt-6 leading-relaxed text-blue-50 max-w-2xl"
								class:text-base={$screenSize.isMobile}
								class:text-lg={!$screenSize.isMobile}
								class:mx-auto={$screenSize.isMobile}
								class:lg:mx-0={true}
							>
								{profile?.bio || 'Créateur d\'applications web performantes et modernes avec une expertise en développement full-stack.'}
							</p>
						</div>

						<div in:slideIn={{ direction: 'up', duration: 600, delay: 900 }}>
							<div 
								class="mt-8 sm:mt-10 flex flex-col gap-4"
								class:sm:flex-row={!$screenSize.isMobile}
								class:items-center={true}
								class:justify-center={$screenSize.isMobile}
								class:lg:justify-start={true}
							>
								<a
									href="/projects"
									class="inline-flex items-center justify-center rounded-md bg-white px-6 py-3 font-semibold text-gray-900 shadow-lg hover:bg-gray-100 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-white transition-all duration-200 transform hover:scale-105 hover:shadow-xl"
									class:w-full={$screenSize.isMobile}
									class:sm:w-auto={true}
									class:text-sm={$screenSize.isMobile}
									class:text-base={!$screenSize.isMobile}
								>
									Découvrir mes projets
								</a>
								<a
									href="/contact"
									class="inline-flex items-center justify-center gap-2 font-semibold leading-6 text-white hover:text-blue-100 transition-colors duration-200 group"
									class:w-full={$screenSize.isMobile}
									class:sm:w-auto={true}
									class:text-sm={$screenSize.isMobile}
									class:text-base={!$screenSize.isMobile}
								>
									Me contacter 
									<svg class="w-4 h-4 transition-transform duration-200 group-hover:translate-x-1" fill="none" stroke="currentColor" viewBox="0 0 24 24">
										<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 8l4 4m0 0l-4 4m4-4H3" />
									</svg>
								</a>
							</div>
						</div>

						<!-- Social Links -->
						{#if profile && (profile.linkedin_url || profile.github_url || profile.twitter_url)}
							<div in:fade={{ duration: 600, delay: 1100 }}>
								<div class="mt-8 flex items-center gap-6">
									<span class="text-sm font-medium text-blue-200">Suivez-moi:</span>
									<div class="flex items-center gap-4">
										{#if profile.linkedin_url}
											<a
												href={profile.linkedin_url}
												target="_blank"
												rel="noopener noreferrer"
												class="text-blue-200 hover:text-white transition-colors duration-200 transform hover:scale-110"
												aria-label="LinkedIn"
											>
												<svg class="w-6 h-6" fill="currentColor" viewBox="0 0 24 24">
													<path d="M20.447 20.452h-3.554v-5.569c0-1.328-.027-3.037-1.852-3.037-1.853 0-2.136 1.445-2.136 2.939v5.667H9.351V9h3.414v1.561h.046c.477-.9 1.637-1.85 3.37-1.85 3.601 0 4.267 2.37 4.267 5.455v6.286zM5.337 7.433c-1.144 0-2.063-.926-2.063-2.065 0-1.138.92-2.063 2.063-2.063 1.14 0 2.064.925 2.064 2.063 0 1.139-.925 2.065-2.064 2.065zm1.782 13.019H3.555V9h3.564v11.452zM22.225 0H1.771C.792 0 0 .774 0 1.729v20.542C0 23.227.792 24 1.771 24h20.451C23.2 24 24 23.227 24 22.271V1.729C24 .774 23.2 0 22.222 0h.003z"/>
												</svg>
											</a>
										{/if}
										{#if profile.github_url}
											<a
												href={profile.github_url}
												target="_blank"
												rel="noopener noreferrer"
												class="text-blue-200 hover:text-white transition-colors duration-200 transform hover:scale-110"
												aria-label="GitHub"
											>
												<svg class="w-6 h-6" fill="currentColor" viewBox="0 0 24 24">
													<path d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z"/>
												</svg>
											</a>
										{/if}
										{#if profile.twitter_url}
											<a
												href={profile.twitter_url}
												target="_blank"
												rel="noopener noreferrer"
												class="text-blue-200 hover:text-white transition-colors duration-200 transform hover:scale-110"
												aria-label="Twitter"
											>
												<svg class="w-6 h-6" fill="currentColor" viewBox="0 0 24 24">
													<path d="M23.953 4.57a10 10 0 01-2.825.775 4.958 4.958 0 002.163-2.723c-.951.555-2.005.959-3.127 1.184a4.92 4.92 0 00-8.384 4.482C7.69 8.095 4.067 6.13 1.64 3.162a4.822 4.822 0 00-.666 2.475c0 1.71.87 3.213 2.188 4.096a4.904 4.904 0 01-2.228-.616v.06a4.923 4.923 0 003.946 4.827 4.996 4.996 0 01-2.212.085 4.936 4.936 0 004.604 3.417 9.867 9.867 0 01-6.102 2.105c-.39 0-.779-.023-1.17-.067a13.995 13.995 0 007.557 2.209c9.053 0 13.998-7.496 13.998-13.985 0-.21 0-.42-.015-.63A9.935 9.935 0 0024 4.59z"/>
												</svg>
											</a>
										{/if}
									</div>
								</div>
							</div>
						{/if}
					</div>
				{/if}
			</div>
		</div>
	</div>
</section>