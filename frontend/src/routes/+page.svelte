<script lang="ts">
	import { onMount } from 'svelte';
	import { fade, fly } from 'svelte/transition';
	import { profile, profileStore } from '$lib/stores/profile';
	import { skillsByCategory, skillsStore } from '$lib/stores/skills';
	import HeroSection from '$lib/components/HeroSection.svelte';
	import SkillsOverview from '$lib/components/SkillsOverview.svelte';

	let heroVisible = false;
	let skillsVisible = false;

	onMount(async () => {
		// Load profile and skills data
		await Promise.all([
			profileStore.loadProfile(),
			skillsStore.loadSkills()
		]);

		// Trigger animations with slight delays
		setTimeout(() => heroVisible = true, 100);
		setTimeout(() => skillsVisible = true, 300);
	});

	// Smooth scroll to sections
	function scrollToSection(sectionId: string) {
		const element = document.getElementById(sectionId);
		if (element) {
			element.scrollIntoView({ behavior: 'smooth' });
		}
	}
</script>

<svelte:head>
	<title>{$profile?.name ? `${$profile.name} - ${$profile.title}` : 'Portfolio - Développeur Web'}</title>
	<meta name="description" content={$profile?.bio || 'Portfolio professionnel d\'un développeur web spécialisé en Rust et Svelte'} />
</svelte:head>

<div class="bg-white">
	<!-- Hero Section -->
	{#if heroVisible}
		<div in:fade={{ duration: 800 }}>
			<HeroSection {profile} />
		</div>
	{/if}

	<!-- Skills Overview Section -->
	<section id="skills" class="py-24 bg-gray-50">
		{#if skillsVisible}
			<div in:fly={{ y: 50, duration: 600, delay: 200 }}>
				<SkillsOverview skillsByCategory={$skillsByCategory} />
			</div>
		{/if}
	</section>

	<!-- Call to Action Section -->
	<section class="py-24 bg-white">
		{#if skillsVisible}
			<div in:fly={{ y: 30, duration: 600, delay: 400 }} class="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8">
				<div class="text-center">
					<h2 class="text-3xl font-bold tracking-tight text-gray-900 sm:text-4xl">
						Prêt à collaborer ?
					</h2>
					<p class="mt-4 text-lg text-gray-600">
						Découvrez mes projets ou contactez-moi pour discuter de votre prochain projet
					</p>
					<div class="mt-10 flex items-center justify-center gap-x-6">
						<a
							href="/projects"
							class="rounded-md bg-blue-600 px-6 py-3 text-base font-semibold text-white shadow-sm hover:bg-blue-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-blue-600 transition-all duration-200 transform hover:scale-105"
						>
							Voir mes projets
						</a>
						<button
							on:click={() => scrollToSection('contact')}
							class="text-base font-semibold leading-6 text-blue-600 hover:text-blue-500 transition-colors duration-200"
						>
							Me contacter <span aria-hidden="true">→</span>
						</button>
					</div>
				</div>
			</div>
		{/if}
	</section>
</div>
