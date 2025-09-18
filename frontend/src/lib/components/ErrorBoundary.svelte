<script lang="ts">
	import { onMount } from 'svelte';
	import { toastStore } from '$lib/stores/toast';

	export let fallback: boolean = true;
	export let showToast: boolean = true;
	export let title: string = 'Une erreur est survenue';
	export let message: string = 'Veuillez réessayer ou contacter le support si le problème persiste.';

	let hasError = false;
	let errorMessage = '';

	// List of error patterns to ignore (IDE-related errors)
	const ignoredErrorPatterns = [
		'Failed to initialize messaging',
		'tx_attempts_exceeded',
		'tx_ack_timeout',
		'host-additional-hooks',
		'getSingleton',
		'connect'
	];

	// Check if error should be ignored
	function shouldIgnoreError(error: any): boolean {
		const errorMessage = error?.message || error?.reason?.message || String(error);
		return ignoredErrorPatterns.some(pattern => 
			errorMessage.toLowerCase().includes(pattern.toLowerCase())
		);
	}

	// Handle JavaScript errors
	onMount(() => {
		const handleError = (event: ErrorEvent) => {
			// Ignore IDE-related errors
			if (shouldIgnoreError(event.error)) {
				console.debug('ErrorBoundary ignored IDE-related error:', event.error?.message);
				return;
			}

			console.error('ErrorBoundary caught application error:', event.error);
			hasError = true;
			errorMessage = event.error?.message || 'Erreur inconnue';
			
			if (showToast) {
				toastStore.error(title, errorMessage);
			}
		};

		const handleUnhandledRejection = (event: PromiseRejectionEvent) => {
			// Ignore IDE-related errors
			if (shouldIgnoreError(event.reason)) {
				console.debug('ErrorBoundary ignored IDE-related promise rejection:', event.reason?.message);
				return;
			}

			console.error('ErrorBoundary caught application promise rejection:', event.reason);
			hasError = true;
			errorMessage = event.reason?.message || 'Erreur de promesse non gérée';
			
			if (showToast) {
				toastStore.error(title, errorMessage);
			}
		};

		window.addEventListener('error', handleError);
		window.addEventListener('unhandledrejection', handleUnhandledRejection);

		return () => {
			window.removeEventListener('error', handleError);
			window.removeEventListener('unhandledrejection', handleUnhandledRejection);
		};
	});

	function retry() {
		hasError = false;
		errorMessage = '';
		// Reload the page to reset state
		window.location.reload();
	}
</script>

{#if hasError && fallback}
	<!-- Error Fallback UI -->
	<div class="min-h-screen flex items-center justify-center bg-gray-50 py-12 px-4 sm:px-6 lg:px-8">
		<div class="max-w-md w-full space-y-8">
			<div class="text-center">
				<div class="mx-auto h-24 w-24 rounded-full bg-red-100 flex items-center justify-center mb-6">
					<svg class="h-12 w-12 text-red-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
						<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-2.5L13.732 4c-.77-.833-1.964-.833-2.732 0L3.732 16.5c-.77.833.192 2.5 1.732 2.5z" />
					</svg>
				</div>
				<h2 class="text-3xl font-bold text-gray-900 mb-4">
					{title}
				</h2>
				<p class="text-gray-600 mb-8">
					{message}
				</p>
				{#if errorMessage}
					<div class="bg-red-50 border border-red-200 rounded-md p-4 mb-6">
						<p class="text-sm text-red-800 font-mono">
							{errorMessage}
						</p>
					</div>
				{/if}
				<div class="flex flex-col sm:flex-row gap-4 justify-center">
					<button
						on:click={retry}
						class="inline-flex items-center gap-2 rounded-md bg-blue-600 px-6 py-3 text-base font-semibold text-white shadow-sm hover:bg-blue-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-blue-600 transition-all duration-200"
					>
						<svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
							<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
						</svg>
						Réessayer
					</button>
					<a
						href="/"
						class="inline-flex items-center gap-2 rounded-md bg-gray-600 px-6 py-3 text-base font-semibold text-white shadow-sm hover:bg-gray-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-gray-600 transition-all duration-200"
					>
						<svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
							<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6" />
						</svg>
						Retour à l'accueil
					</a>
				</div>
			</div>
		</div>
	</div>
{:else}
	<!-- Normal content -->
	<slot />
{/if}