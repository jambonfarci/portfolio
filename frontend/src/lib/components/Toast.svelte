<script lang="ts">
	import { fly, fade } from 'svelte/transition';
	import { toasts, toastStore, type Toast } from '$lib/stores/toast';

	export let toast: Toast;

	function dismiss() {
		toastStore.dismiss(toast.id);
	}

	// Icon mapping for different toast types
	const icons = {
		success: 'M5 13l4 4L19 7',
		error: 'M6 18L18 6M6 6l12 12',
		warning: 'M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-2.5L13.732 4c-.77-.833-1.964-.833-2.732 0L3.732 16.5c-.77.833.192 2.5 1.732 2.5z',
		info: 'M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z'
	};

	// Color classes for different toast types
	const colorClasses = {
		success: 'bg-green-50 border-green-200 text-green-800',
		error: 'bg-red-50 border-red-200 text-red-800',
		warning: 'bg-yellow-50 border-yellow-200 text-yellow-800',
		info: 'bg-blue-50 border-blue-200 text-blue-800'
	};

	const iconColorClasses = {
		success: 'text-green-400',
		error: 'text-red-400',
		warning: 'text-yellow-400',
		info: 'text-blue-400'
	};
</script>

<div
	in:fly={{ x: 300, duration: 300 }}
	out:fly={{ x: 300, duration: 200 }}
	class="pointer-events-auto w-full max-w-sm overflow-hidden rounded-lg shadow-lg ring-1 ring-black ring-opacity-5 {colorClasses[toast.type]}"
>
	<div class="p-4">
		<div class="flex items-start">
			<div class="flex-shrink-0">
				<svg 
					class="h-6 w-6 {iconColorClasses[toast.type]}" 
					fill="none" 
					stroke="currentColor" 
					viewBox="0 0 24 24"
					stroke-width="2"
					stroke-linecap="round"
					stroke-linejoin="round"
				>
					<path d={icons[toast.type]} />
				</svg>
			</div>
			<div class="ml-3 w-0 flex-1 pt-0.5">
				<p class="text-sm font-medium">
					{toast.title}
				</p>
				{#if toast.message}
					<p class="mt-1 text-sm opacity-90">
						{toast.message}
					</p>
				{/if}
			</div>
			{#if toast.dismissible}
				<div class="ml-4 flex flex-shrink-0">
					<button
						type="button"
						class="inline-flex rounded-md focus:outline-none focus:ring-2 focus:ring-offset-2 hover:opacity-75 transition-opacity duration-150"
						class:focus:ring-green-500={toast.type === 'success'}
						class:focus:ring-red-500={toast.type === 'error'}
						class:focus:ring-yellow-500={toast.type === 'warning'}
						class:focus:ring-blue-500={toast.type === 'info'}
						on:click={dismiss}
					>
						<span class="sr-only">Fermer</span>
						<svg class="h-5 w-5 opacity-60" fill="none" stroke="currentColor" viewBox="0 0 24 24">
							<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
						</svg>
					</button>
				</div>
			{/if}
		</div>
	</div>
</div>