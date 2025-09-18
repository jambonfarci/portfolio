<script lang="ts">
	import { toasts } from '$lib/stores/toast';
	import Toast from './Toast.svelte';
</script>

<!-- Toast Container - Fixed position at top-right -->
<div class="fixed top-4 right-4 z-50 flex flex-col gap-2 pointer-events-none">
	{#each $toasts as toast (toast.id)}
		<Toast {toast} />
	{/each}
</div>