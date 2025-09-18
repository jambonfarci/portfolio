<script lang="ts">
	import { onMount } from 'svelte';
	import { fade, fly } from 'svelte/transition';
	import { profile, profileStore } from '$lib/stores/profile';
	import ContactForm from '$lib/components/ContactForm.svelte';
	import SocialLinks from '$lib/components/SocialLinks.svelte';

	let pageVisible = false;
	let showSuccessMessage = false;

	onMount(async () => {
		// Load profile data
		await profileStore.loadProfile();
		
		// Trigger page animation
		setTimeout(() => pageVisible = true, 100);
	});

	function handleContactSuccess(event: CustomEvent) {
		showSuccessMessage = true;
		setTimeout(() => {
			showSuccessMessage = false;
		}, 5000);
	}
</script>

<svelte:head>
	<title>Contact - Portfolio Développeur Web</title>
	<meta name="description" content="Contactez-moi pour discuter de votre projet de développement web. Je suis disponible pour des missions freelance et des collaborations." />
</svelte:head>

<div class="min-h-screen bg-white">
	<!-- Hero Section -->
	{#if pageVisible}
		<section in:fade={{ duration: 800 }} class="relative bg-gradient-to-br from-blue-50 to-indigo-100 py-16 sm:py-24">
			<div class="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8">
				<div in:fly={{ y: 30, duration: 600, delay: 200 }} class="text-center">
					<h1 class="text-4xl font-bold tracking-tight text-gray-900 sm:text-5xl lg:text-6xl">
						Contactez-moi
					</h1>
					<p class="mt-6 text-lg leading-8 text-gray-600 max-w-2xl mx-auto">
						Vous avez un projet en tête ? Une question ? N'hésitez pas à me contacter. 
						Je serais ravi de discuter avec vous de vos besoins en développement web.
					</p>
				</div>
			</div>
			
			<!-- Decorative elements -->
			<div class="absolute top-0 left-0 w-full h-full overflow-hidden pointer-events-none">
				<div class="absolute top-1/4 left-1/4 w-32 h-32 bg-blue-200 opacity-20 rounded-full blur-xl"></div>
				<div class="absolute bottom-1/4 right-1/4 w-48 h-48 bg-indigo-200 opacity-20 rounded-full blur-xl"></div>
			</div>
		</section>
	{/if}

	<!-- Success Message -->
	{#if showSuccessMessage}
		<div in:fly={{ y: -50, duration: 400 }} out:fly={{ y: -50, duration: 300 }} class="fixed top-4 right-4 z-50">
			<div class="rounded-md bg-green-50 p-4 shadow-lg ring-1 ring-green-200">
				<div class="flex">
					<div class="flex-shrink-0">
						<svg class="h-5 w-5 text-green-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
							<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
						</svg>
					</div>
					<div class="ml-3">
						<p class="text-sm font-medium text-green-800">
							Message envoyé avec succès !
						</p>
					</div>
				</div>
			</div>
		</div>
	{/if}

	<!-- Contact Section -->
	<section class="py-16 sm:py-24">
		<div class="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8">
			<div class="lg:grid lg:grid-cols-2 lg:gap-16">
				<!-- Contact Information -->
				<div class="mb-12 lg:mb-0">
					{#if pageVisible}
						<div in:fly={{ x: -50, duration: 600, delay: 300 }}>
							<h2 class="text-2xl font-bold text-gray-900 sm:text-3xl mb-6">
								Informations de contact
							</h2>
							
							{#if $profile}
								<div class="space-y-6">
									<!-- Email -->
									<div class="flex items-center gap-4">
										<div class="flex h-12 w-12 items-center justify-center rounded-lg bg-blue-100">
											<svg class="h-6 w-6 text-blue-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
												<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 8l7.89 4.26a2 2 0 002.22 0L21 8M5 19h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z" />
											</svg>
										</div>
										<div>
											<p class="text-sm font-medium text-gray-900">Email</p>
											<a href="mailto:{$profile.email}" class="text-blue-600 hover:text-blue-500 transition-colors duration-200">
												{$profile.email}
											</a>
										</div>
									</div>

									<!-- Phone -->
									{#if $profile.phone}
										<div class="flex items-center gap-4">
											<div class="flex h-12 w-12 items-center justify-center rounded-lg bg-green-100">
												<svg class="h-6 w-6 text-green-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
													<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 5a2 2 0 012-2h3.28a1 1 0 01.948.684l1.498 4.493a1 1 0 01-.502 1.21l-2.257 1.13a11.042 11.042 0 005.516 5.516l1.13-2.257a1 1 0 011.21-.502l4.493 1.498a1 1 0 01.684.949V19a2 2 0 01-2 2h-1C9.716 21 3 14.284 3 6V5z" />
												</svg>
											</div>
											<div>
												<p class="text-sm font-medium text-gray-900">Téléphone</p>
												<a href="tel:{$profile.phone}" class="text-green-600 hover:text-green-500 transition-colors duration-200">
													{$profile.phone}
												</a>
											</div>
										</div>
									{/if}

									<!-- Location -->
									<div class="flex items-center gap-4">
										<div class="flex h-12 w-12 items-center justify-center rounded-lg bg-purple-100">
											<svg class="h-6 w-6 text-purple-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
												<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17.657 16.657L13.414 20.9a1.998 1.998 0 01-2.827 0l-4.244-4.243a8 8 0 1111.314 0z" />
												<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 11a3 3 0 11-6 0 3 3 0 016 0z" />
											</svg>
										</div>
										<div>
											<p class="text-sm font-medium text-gray-900">Localisation</p>
											<p class="text-gray-600">{$profile.location}</p>
										</div>
									</div>
								</div>

								<!-- Social Links -->
								<div class="mt-8">
									<h3 class="text-lg font-semibold text-gray-900 mb-4">Suivez-moi</h3>
									<SocialLinks profile={$profile} size="md" showLabels={true} />
								</div>
							{:else}
								<!-- Loading state -->
								<div class="space-y-4">
									<div class="animate-pulse">
										<div class="h-4 bg-gray-200 rounded w-3/4 mb-2"></div>
										<div class="h-4 bg-gray-200 rounded w-1/2"></div>
									</div>
								</div>
							{/if}

							<!-- Additional Info -->
							<div class="mt-8 p-6 bg-gray-50 rounded-lg">
								<h3 class="text-lg font-semibold text-gray-900 mb-3">Disponibilité</h3>
								<p class="text-gray-600 mb-4">
									Je suis actuellement disponible pour de nouveaux projets et collaborations. 
									N'hésitez pas à me contacter pour discuter de vos besoins.
								</p>
								<div class="flex items-center gap-2 text-sm text-green-600">
									<div class="w-2 h-2 bg-green-500 rounded-full"></div>
									<span>Disponible pour de nouveaux projets</span>
								</div>
							</div>
						</div>
					{/if}
				</div>

				<!-- Contact Form -->
				<div>
					{#if pageVisible}
						<div in:fly={{ x: 50, duration: 600, delay: 500 }}>
							<h2 class="text-2xl font-bold text-gray-900 sm:text-3xl mb-6">
								Envoyez-moi un message
							</h2>
							<div class="bg-white p-6 rounded-lg shadow-lg ring-1 ring-gray-200">
								<ContactForm on:success={handleContactSuccess} />
							</div>
						</div>
					{/if}
				</div>
			</div>
		</div>
	</section>

	<!-- FAQ Section -->
	{#if pageVisible}
		<section in:fly={{ y: 30, duration: 600, delay: 700 }} class="py-16 bg-gray-50">
			<div class="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8">
				<div class="text-center mb-12">
					<h2 class="text-3xl font-bold tracking-tight text-gray-900 sm:text-4xl">
						Questions fréquentes
					</h2>
					<p class="mt-4 text-lg text-gray-600">
						Voici les réponses aux questions les plus courantes
					</p>
				</div>

				<div class="grid gap-8 md:grid-cols-2">
					<div class="bg-white p-6 rounded-lg shadow-sm">
						<h3 class="text-lg font-semibold text-gray-900 mb-3">
							Quel est votre délai de réponse ?
						</h3>
						<p class="text-gray-600">
							Je réponds généralement aux messages dans les 24 heures ouvrées. 
							Pour les demandes urgentes, n'hésitez pas à le mentionner dans votre message.
						</p>
					</div>

					<div class="bg-white p-6 rounded-lg shadow-sm">
						<h3 class="text-lg font-semibold text-gray-900 mb-3">
							Quels types de projets acceptez-vous ?
						</h3>
						<p class="text-gray-600">
							Je travaille sur des applications web modernes, des API REST, 
							et des solutions full-stack utilisant Rust, Svelte, et d'autres technologies récentes.
						</p>
					</div>

					<div class="bg-white p-6 rounded-lg shadow-sm">
						<h3 class="text-lg font-semibold text-gray-900 mb-3">
							Proposez-vous de la maintenance ?
						</h3>
						<p class="text-gray-600">
							Oui, je propose des services de maintenance et d'évolution 
							pour les projets que je développe, ainsi que pour des projets existants.
						</p>
					</div>

					<div class="bg-white p-6 rounded-lg shadow-sm">
						<h3 class="text-lg font-semibold text-gray-900 mb-3">
							Travaillez-vous en équipe ?
						</h3>
						<p class="text-gray-600">
							Je peux travailler seul ou intégrer une équipe existante. 
							Je suis habitué aux méthodologies agiles et aux outils de collaboration modernes.
						</p>
					</div>
				</div>
			</div>
		</section>
	{/if}
</div>