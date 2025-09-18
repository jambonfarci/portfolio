<script lang="ts">
	import '../app.css';
	import favicon from '$lib/assets/favicon.svg';
	import Header from '$lib/components/Header.svelte';
	import Footer from '$lib/components/Footer.svelte';
	import ToastContainer from '$lib/components/ToastContainer.svelte';
	import ErrorBoundary from '$lib/components/ErrorBoundary.svelte';
	import ErrorDebugPanel from '$lib/components/ErrorDebugPanel.svelte';

	let { children } = $props();
</script>

<svelte:head>
	<link rel="icon" href={favicon} />
	<meta name="viewport" content="width=device-width, initial-scale=1" />
</svelte:head>

<ErrorBoundary>
	<div class="min-h-screen flex flex-col">
		<Header />
		
		<main class="flex-1">
			{@render children?.()}
		</main>
		
		<Footer />
	</div>
	
	<!-- Toast notifications -->
	<ToastContainer />
	
	<!-- Error debug panel (development only) -->
	<ErrorDebugPanel />
</ErrorBoundary>
