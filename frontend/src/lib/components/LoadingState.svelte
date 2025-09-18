<script lang="ts">
	import LoadingSpinner from './LoadingSpinner.svelte';

	export let text: string = 'Chargement...';
	export let fullScreen: boolean = false;
	export let overlay: boolean = false;
</script>

<div 
	class="flex items-center justify-center"
	class:min-h-screen={fullScreen}
	class:py-12={!fullScreen}
	class:fixed={overlay}
	class:inset-0={overlay}
	class:bg-white={overlay}
	class:bg-opacity-75={overlay}
	class:z-50={overlay}
>
	<div class="text-center">
		<LoadingSpinner size="lg" {text} center />
	</div>
</div>