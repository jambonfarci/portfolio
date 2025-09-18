<script lang="ts">
	import { fly, fade } from 'svelte/transition';
	import type { Skill } from '$lib/types';

	export let skillsByCategory: Record<string, Skill[]>;

	let sectionVisible = false;
	let categoriesVisible: Record<string, boolean> = {};

	// Trigger animations
	setTimeout(() => {
		sectionVisible = true;
		// Stagger category animations
		Object.keys(skillsByCategory).forEach((category, index) => {
			setTimeout(() => {
				categoriesVisible[category] = true;
			}, index * 150);
		});
	}, 100);

	// Get skill level display
	function getSkillLevelDisplay(level: number): string {
		const levels = ['Débutant', 'Intermédiaire', 'Confirmé', 'Expert', 'Maître'];
		return levels[level - 1] || 'Débutant';
	}

	// Get skill level color
	function getSkillLevelColor(level: number): string {
		const colors = [
			'bg-red-100 text-red-800',
			'bg-yellow-100 text-yellow-800', 
			'bg-blue-100 text-blue-800',
			'bg-green-100 text-green-800',
			'bg-purple-100 text-purple-800'
		];
		return colors[level - 1] || colors[0];
	}

	// Get category icon
	function getCategoryIcon(category: string): string {
		const icons: Record<string, string> = {
			'Frontend': '🎨',
			'Backend': '⚙️',
			'Database': '🗄️',
			'DevOps': '🚀',
			'Tools': '🔧',
			'Mobile': '📱',
			'Design': '✨'
		};
		return icons[category] || '💻';
	}

	// Get category color
	function getCategoryColor(category: string): string {
		const colors: Record<string, string> = {
			'Frontend': 'from-blue-500 to-cyan-500',
			'Backend': 'from-green-500 to-emerald-500',
			'Database': 'from-purple-500 to-violet-500',
			'DevOps': 'from-orange-500 to-red-500',
			'Tools': 'from-gray-500 to-slate-500',
			'Mobile': 'from-pink-500 to-rose-500',
			'Design': 'from-indigo-500 to-purple-500'
		};
		return colors[category] || 'from-gray-500 to-slate-500';
	}
</script>

<div class="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8">
	{#if sectionVisible}
		<div in:fade={{ duration: 600 }}>
			<div class="text-center">
				<h2 class="text-3xl font-bold tracking-tight text-gray-900 sm:text-4xl">
					Compétences Techniques
				</h2>
				<p class="mt-4 text-lg text-gray-600 max-w-2xl mx-auto">
					Technologies et outils que je maîtrise pour créer des solutions innovantes et performantes
				</p>
			</div>
		</div>
	{/if}

	{#if Object.keys(skillsByCategory).length > 0}
		<div class="mt-16 grid gap-8 md:grid-cols-2 lg:grid-cols-3">
			{#each Object.entries(skillsByCategory) as [category, skills]}
				{#if categoriesVisible[category]}
					<div in:fly={{ y: 30, duration: 600 }} class="group">
						<div class="relative overflow-hidden rounded-2xl bg-white p-6 shadow-lg ring-1 ring-gray-200 transition-all duration-300 hover:shadow-xl hover:ring-gray-300">
							<!-- Category Header -->
							<div class="flex items-center gap-3 mb-6">
								<div class="flex h-12 w-12 items-center justify-center rounded-xl bg-gradient-to-r {getCategoryColor(category)} text-white text-xl">
									{getCategoryIcon(category)}
								</div>
								<div>
									<h3 class="text-lg font-semibold text-gray-900">{category}</h3>
									<p class="text-sm text-gray-500">{skills.length} compétence{skills.length > 1 ? 's' : ''}</p>
								</div>
							</div>

							<!-- Skills List -->
							<div class="space-y-3">
								{#each skills as skill, index}
									<div 
										in:fade={{ duration: 400, delay: index * 50 }}
										class="flex items-center justify-between p-3 rounded-lg bg-gray-50 hover:bg-gray-100 transition-colors duration-200"
									>
										<div class="flex-1">
											<div class="flex items-center gap-2">
												<span class="font-medium text-gray-900">{skill.name}</span>
												{#if skill.years_experience}
													<span class="text-xs text-gray-500">
														({skill.years_experience} an{skill.years_experience > 1 ? 's' : ''})
													</span>
												{/if}
											</div>
											{#if skill.description}
												<p class="text-xs text-gray-600 mt-1">{skill.description}</p>
											{/if}
										</div>
										<div class="ml-3">
											<span class="inline-flex items-center rounded-full px-2 py-1 text-xs font-medium {getSkillLevelColor(skill.level)}">
												{getSkillLevelDisplay(skill.level)}
											</span>
										</div>
									</div>
								{/each}
							</div>

							<!-- Decorative gradient overlay -->
							<div class="absolute inset-0 rounded-2xl bg-gradient-to-r {getCategoryColor(category)} opacity-0 group-hover:opacity-5 transition-opacity duration-300"></div>
						</div>
					</div>
				{/if}
			{/each}
		</div>
	{:else}
		{#if sectionVisible}
			<div in:fade={{ duration: 600, delay: 300 }} class="mt-16 text-center">
				<div class="mx-auto max-w-md">
					<div class="flex justify-center mb-4">
						<div class="h-16 w-16 rounded-full bg-gray-100 flex items-center justify-center">
							<svg class="h-8 w-8 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
								<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19.428 15.428a2 2 0 00-1.022-.547l-2.387-.477a6 6 0 00-3.86.517l-.318.158a6 6 0 01-3.86.517L6.05 15.21a2 2 0 00-1.806.547M8 4h8l-1 1v5.172a2 2 0 00.586 1.414l5 5c1.26 1.26.367 3.414-1.415 3.414H4.828c-1.782 0-2.674-2.154-1.414-3.414l5-5A2 2 0 009 7.172V5L8 4z" />
							</svg>
						</div>
					</div>
					<h3 class="text-lg font-medium text-gray-900 mb-2">Compétences en cours de chargement</h3>
					<p class="text-gray-500">
						Les compétences techniques seront affichées une fois les données chargées depuis le backend.
					</p>
				</div>
			</div>
		{/if}
	{/if}

	<!-- Call to action -->
	{#if sectionVisible && Object.keys(skillsByCategory).length > 0}
		<div in:fly={{ y: 20, duration: 600, delay: 800 }} class="mt-12 text-center">
			<p class="text-gray-600 mb-6">
				Intéressé par une collaboration ? Découvrez comment ces compétences peuvent servir votre projet.
			</p>
			<a
				href="/contact"
				class="inline-flex items-center gap-2 rounded-md bg-blue-600 px-4 py-2 text-sm font-semibold text-white shadow-sm hover:bg-blue-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-blue-600 transition-all duration-200 transform hover:scale-105"
			>
				Discutons de votre projet
				<svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
					<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 8l4 4m0 0l-4 4m4-4H3" />
				</svg>
			</a>
		</div>
	{/if}
</div>