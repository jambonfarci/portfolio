<script lang="ts">
	import { onMount } from 'svelte';
	import { fade } from 'svelte/transition';

	export let src: string;
	export let alt: string;
	export let width: number | undefined = undefined;
	export let height: number | undefined = undefined;
	export let sizes: string = '100vw';
	export let loading: 'lazy' | 'eager' = 'lazy';
	let className: string = '';
	export { className as class };
	export let placeholder: string = '';
	export let quality: number = 80;

	let imageElement: HTMLImageElement;
	let isLoaded = false;
	let isInView = false;
	let hasError = false;
	let observer: IntersectionObserver;

	// Generate responsive image URLs with different sizes
	function generateSrcSet(baseSrc: string): string {
		const sizes = [320, 640, 768, 1024, 1280, 1920];
		return sizes
			.map(size => `${baseSrc}?w=${size}&q=${quality} ${size}w`)
			.join(', ');
	}

	// Generate optimized src URL
	function getOptimizedSrc(baseSrc: string): string {
		if (width) {
			return `${baseSrc}?w=${width}&q=${quality}`;
		}
		return `${baseSrc}?q=${quality}`;
	}

	// Intersection Observer for lazy loading
	onMount(() => {
		if (loading === 'lazy' && imageElement) {
			observer = new IntersectionObserver(
				(entries) => {
					entries.forEach((entry) => {
						if (entry.isIntersecting) {
							isInView = true;
							observer.unobserve(entry.target);
						}
					});
				},
				{
					rootMargin: '50px'
				}
			);

			observer.observe(imageElement);
		} else {
			isInView = true;
		}

		return () => {
			if (observer) {
				observer.disconnect();
			}
		};
	});

	function handleLoad() {
		isLoaded = true;
	}

	function handleError() {
		hasError = true;
		isLoaded = true;
	}

	// Preload critical images
	function preloadImage(src: string) {
		const link = document.createElement('link');
		link.rel = 'preload';
		link.as = 'image';
		link.href = src;
		document.head.appendChild(link);
	}

	$: if (loading === 'eager' && src) {
		preloadImage(getOptimizedSrc(src));
	}
</script>

<div class="relative overflow-hidden {className}" bind:this={imageElement}>
	<!-- Placeholder while loading -->
	{#if !isLoaded && placeholder}
		<div class="absolute inset-0 bg-gray-200 animate-pulse flex items-center justify-center">
			<div class="text-gray-400">
				<svg class="w-8 h-8" fill="none" stroke="currentColor" viewBox="0 0 24 24">
					<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z" />
				</svg>
			</div>
		</div>
	{/if}

	<!-- Actual image -->
	{#if isInView && !hasError}
		<img
			src={getOptimizedSrc(src)}
			srcset={generateSrcSet(src)}
			{sizes}
			{alt}
			{width}
			{height}
			class="w-full h-full object-cover transition-opacity duration-300"
			class:opacity-0={!isLoaded}
			class:opacity-100={isLoaded}
			on:load={handleLoad}
			on:error={handleError}
			loading={loading}
			decoding="async"
		/>
	{/if}

	<!-- Error fallback -->
	{#if hasError}
		<div class="absolute inset-0 bg-gray-100 flex items-center justify-center">
			<div class="text-center text-gray-500">
				<svg class="w-12 h-12 mx-auto mb-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
					<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-2.5L13.732 4c-.77-.833-1.964-.833-2.732 0L4.082 16.5c-.77.833.192 2.5 1.732 2.5z" />
				</svg>
				<p class="text-sm">Image non disponible</p>
			</div>
		</div>
	{/if}

	<!-- Loading indicator -->
	{#if !isLoaded && !hasError && isInView}
		<div class="absolute inset-0 bg-gray-200 animate-pulse"></div>
	{/if}
</div>