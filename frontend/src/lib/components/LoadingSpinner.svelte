<script lang="ts">
	export let size: 'sm' | 'md' | 'lg' | 'xl' = 'md';
	export let color: 'blue' | 'gray' | 'white' = 'blue';
	export let text: string = '';
	export let center: boolean = false;

	const sizeClasses = {
		sm: 'h-4 w-4',
		md: 'h-8 w-8',
		lg: 'h-12 w-12',
		xl: 'h-16 w-16'
	};

	const colorClasses = {
		blue: 'border-blue-600',
		gray: 'border-gray-600',
		white: 'border-white'
	};

	const textSizeClasses = {
		sm: 'text-sm',
		md: 'text-base',
		lg: 'text-lg',
		xl: 'text-xl'
	};
</script>

<div class="flex items-center gap-3" class:justify-center={center}>
	<div
		class="animate-spin rounded-full border-2 border-t-transparent {sizeClasses[size]} {colorClasses[color]}"
		role="status"
		aria-label="Chargement en cours"
	></div>
	{#if text}
		<span class="text-gray-600 {textSizeClasses[size]}">
			{text}
		</span>
	{/if}
</div>