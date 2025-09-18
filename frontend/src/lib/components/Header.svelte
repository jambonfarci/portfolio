<script lang="ts">
	import { page } from '$app/stores';
	
	let mobileMenuOpen = $state(false);
	
	const navigation = [
		{ name: 'Accueil', href: '/' },
		{ name: 'Projets', href: '/projects' },
		{ name: 'À propos', href: '/about' },
		{ name: 'Contact', href: '/contact' }
	];
	
	function toggleMobileMenu() {
		mobileMenuOpen = !mobileMenuOpen;
	}
	
	function closeMobileMenu() {
		mobileMenuOpen = false;
	}
</script>

<header class="bg-white shadow-sm border-b border-gray-200">
	<nav class="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8">
		<div class="flex h-16 justify-between">
			<!-- Logo/Brand -->
			<div class="flex items-center">
				<a href="/" class="flex items-center">
					<span class="text-xl font-bold text-gray-900">Portfolio</span>
				</a>
			</div>

			<!-- Desktop Navigation -->
			<div class="hidden md:flex md:items-center md:space-x-8">
				{#each navigation as item}
					<a
						href={item.href}
						class="text-gray-700 hover:text-gray-900 px-3 py-2 text-sm font-medium transition-colors duration-200"
						class:text-gray-900={$page.url.pathname === item.href}
						class:font-semibold={$page.url.pathname === item.href}
					>
						{item.name}
					</a>
				{/each}
			</div>

			<!-- Mobile menu button -->
			<div class="md:hidden flex items-center">
				<button
					type="button"
					class="text-gray-700 hover:text-gray-900 focus:outline-none focus:ring-2 focus:ring-inset focus:ring-blue-500 p-2"
					onclick={toggleMobileMenu}
					aria-expanded={mobileMenuOpen}
					aria-label="Toggle navigation menu"
				>
					<svg class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor">
						{#if mobileMenuOpen}
							<path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12" />
						{:else}
							<path stroke-linecap="round" stroke-linejoin="round" d="M3.75 6.75h16.5M3.75 12h16.5m-16.5 5.25h16.5" />
						{/if}
					</svg>
				</button>
			</div>
		</div>

		<!-- Mobile Navigation Menu -->
		{#if mobileMenuOpen}
			<div class="md:hidden">
				<div class="px-2 pt-2 pb-3 space-y-1 sm:px-3 border-t border-gray-200">
					{#each navigation as item}
						<a
							href={item.href}
							class="block px-3 py-2 text-base font-medium text-gray-700 hover:text-gray-900 hover:bg-gray-50 rounded-md transition-colors duration-200"
							class:text-gray-900={$page.url.pathname === item.href}
							class:bg-gray-100={$page.url.pathname === item.href}
							onclick={closeMobileMenu}
						>
							{item.name}
						</a>
					{/each}
				</div>
			</div>
		{/if}
	</nav>
</header>