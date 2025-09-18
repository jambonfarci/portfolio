<script lang="ts">
	import { fade, scale } from 'svelte/transition';
	import type { Skill } from '$lib/types';

	export let skill: Skill;
	export let showLevel = true;
	export let showExperience = true;
	export let showDescription = false;
	export let size: 'sm' | 'md' | 'lg' = 'md';
	export let interactive = true;

	let isHovered = false;
	let badgeVisible = false;

	// Trigger animation
	setTimeout(() => badgeVisible = true, 100);

	// Get skill level display
	function getSkillLevelDisplay(level: number): string {
		const levels = ['Débutant', 'Intermédiaire', 'Confirmé', 'Expert', 'Maître'];
		return levels[level - 1] || 'Débutant';
	}

	// Get skill level color
	function getSkillLevelColor(level: number): string {
		const colors = [
			'bg-red-100 text-red-800 ring-red-600/20',
			'bg-yellow-100 text-yellow-800 ring-yellow-600/20', 
			'bg-blue-100 text-blue-800 ring-blue-600/20',
			'bg-green-100 text-green-800 ring-green-600/20',
			'bg-purple-100 text-purple-800 ring-purple-600/20'
		];
		return colors[level - 1] || colors[0];
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

	// Get size classes
	function getSizeClasses(size: string): string {
		const sizes = {
			'sm': 'px-2 py-1 text-xs',
			'md': 'px-3 py-2 text-sm',
			'lg': 'px-4 py-3 text-base'
		};
		return sizes[size as keyof typeof sizes] || sizes.md;
	}

	// Generate level indicators (stars or dots)
	function getLevelIndicators(level: number): string[] {
		return Array.from({ length: 5 }, (_, i) => i < level ? 'filled' : 'empty');
	}
</script>

{#if badgeVisible}
	<div
		in:scale={{ duration: 300, start: 0.8 }}
		class="group relative inline-flex items-center gap-2 rounded-lg bg-white shadow-sm ring-1 ring-gray-200 transition-all duration-200 {getSizeClasses(size)}"
		class:hover:shadow-md={interactive}
		class:hover:ring-gray-300={interactive}
		class:hover:-translate-y-0.5={interactive}
		class:cursor-pointer={interactive}
		on:mouseenter={() => isHovered = true}
		on:mouseleave={() => isHovered = false}
		role={interactive ? 'button' : undefined}
		tabindex={interactive ? 0 : undefined}
	>
		<!-- Skill Name -->
		<div class="flex items-center gap-2">
			<span class="font-medium text-gray-900">
				{skill.name}
			</span>
			
			{#if showExperience && skill.years_experience}
				<span class="text-xs text-gray-500">
					({skill.years_experience} an{skill.years_experience > 1 ? 's' : ''})
				</span>
			{/if}
		</div>

		<!-- Level Indicators -->
		{#if showLevel}
			<div class="flex items-center gap-1">
				{#each getLevelIndicators(skill.level) as indicator}
					<div 
						class="w-2 h-2 rounded-full transition-colors duration-200"
						class:bg-yellow-400={indicator === 'filled'}
						class:bg-gray-200={indicator === 'empty'}
					></div>
				{/each}
			</div>
		{/if}

		<!-- Level Badge -->
		{#if showLevel}
			<span class="inline-flex items-center rounded-full px-2 py-0.5 text-xs font-medium ring-1 ring-inset {getSkillLevelColor(skill.level)}">
				{getSkillLevelDisplay(skill.level)}
			</span>
		{/if}

		<!-- Hover Tooltip -->
		{#if interactive && isHovered && (skill.description || showDescription)}
			<div 
				in:fade={{ duration: 200 }}
				out:fade={{ duration: 150 }}
				class="absolute bottom-full left-1/2 transform -translate-x-1/2 mb-2 z-10"
			>
				<div class="bg-gray-900 text-white text-xs rounded-lg px-3 py-2 max-w-xs shadow-lg">
					<div class="text-center">
						<div class="font-medium mb-1">{skill.name}</div>
						<div class="text-gray-300 mb-2">
							{getSkillLevelDisplay(skill.level)} • {skill.category}
						</div>
						{#if skill.description}
							<div class="text-gray-200 text-xs">
								{skill.description}
							</div>
						{/if}
						{#if skill.years_experience}
							<div class="text-gray-300 text-xs mt-1">
								{skill.years_experience} année{skill.years_experience > 1 ? 's' : ''} d'expérience
							</div>
						{/if}
					</div>
					<!-- Tooltip arrow -->
					<div class="absolute top-full left-1/2 transform -translate-x-1/2 w-0 h-0 border-l-4 border-r-4 border-t-4 border-transparent border-t-gray-900"></div>
				</div>
			</div>
		{/if}

		<!-- Category gradient overlay -->
		<div class="absolute inset-0 rounded-lg bg-gradient-to-r {getCategoryColor(skill.category)} opacity-0 group-hover:opacity-5 transition-opacity duration-300 pointer-events-none"></div>
	</div>
{/if}