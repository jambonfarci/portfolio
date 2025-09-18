<script lang="ts">
	import { fade, fly } from 'svelte/transition';
	import { createEventDispatcher } from 'svelte';
	import type { ContactMessage } from '$lib/types';
	import { contactStore, contactLoading, contactSuccess } from '$lib/stores/contact';
	import LoadingSpinner from './LoadingSpinner.svelte';

	const dispatch = createEventDispatcher();

	// Form state
	let formData: ContactMessage = {
		name: '',
		email: '',
		subject: '',
		message: ''
	};

	// Remove local state, use store instead
	let formVisible = false;

	// Validation state
	let errors: Partial<Record<keyof ContactMessage, string>> = {};
	let touched: Partial<Record<keyof ContactMessage, boolean>> = {};

	// Trigger animation
	setTimeout(() => formVisible = true, 200);

	// Validation rules
	function validateField(field: keyof ContactMessage, value: string): string | null {
		switch (field) {
			case 'name':
				if (!value.trim()) return 'Le nom est requis';
				if (value.trim().length < 2) return 'Le nom doit contenir au moins 2 caractères';
				return null;
			
			case 'email':
				if (!value.trim()) return 'L\'email est requis';
				const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
				if (!emailRegex.test(value)) return 'Format d\'email invalide';
				return null;
			
			case 'subject':
				if (!value.trim()) return 'Le sujet est requis';
				if (value.trim().length < 5) return 'Le sujet doit contenir au moins 5 caractères';
				return null;
			
			case 'message':
				if (!value.trim()) return 'Le message est requis';
				if (value.trim().length < 10) return 'Le message doit contenir au moins 10 caractères';
				if (value.trim().length > 1000) return 'Le message ne peut pas dépasser 1000 caractères';
				return null;
			
			default:
				return null;
		}
	}

	// Handle field blur
	function handleBlur(field: keyof ContactMessage) {
		touched[field] = true;
		const error = validateField(field, formData[field]);
		if (error) {
			errors[field] = error;
		} else {
			delete errors[field];
		}
		errors = { ...errors };
	}

	// Handle input change
	function handleInput(field: keyof ContactMessage, value: string) {
		formData[field] = value;
		
		// Clear error if field becomes valid
		if (touched[field]) {
			const error = validateField(field, value);
			if (error) {
				errors[field] = error;
			} else {
				delete errors[field];
			}
			errors = { ...errors };
		}
	}

	// Validate entire form
	function validateForm(): boolean {
		const newErrors: Partial<Record<keyof ContactMessage, string>> = {};
		
		Object.keys(formData).forEach(key => {
			const field = key as keyof ContactMessage;
			const error = validateField(field, formData[field]);
			if (error) {
				newErrors[field] = error;
			}
		});
		
		errors = newErrors;
		return Object.keys(errors).length === 0;
	}

	// Handle form submission
	async function handleSubmit() {
		// Mark all fields as touched
		Object.keys(formData).forEach(key => {
			touched[key as keyof ContactMessage] = true;
		});
		
		if (!validateForm()) {
			return;
		}

		const success = await contactStore.sendMessage(formData);
		
		if (success) {
			// Reset form
			formData = {
				name: '',
				email: '',
				subject: '',
				message: ''
			};
			touched = {};
			errors = {};
			
			dispatch('success', { message: 'Message envoyé avec succès!' });
		}
	}

	// Reset success state
	function resetSuccess() {
		contactStore.clearSuccess();
	}

	// Character count for message
	$: messageCharCount = formData.message.length;
	$: messageCharLimit = 1000;
</script>

{#if formVisible}
	<div in:fade={{ duration: 600 }}>
		{#if $contactSuccess}
			<!-- Success State -->
			<div in:fly={{ y: 20, duration: 400 }} class="text-center py-8">
				<div class="mx-auto flex items-center justify-center w-16 h-16 rounded-full bg-green-100 mb-4">
					<svg class="w-8 h-8 text-green-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
						<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
					</svg>
				</div>
				<h3 class="text-lg font-semibold text-gray-900 mb-2">Message envoyé !</h3>
				<p class="text-gray-600 mb-6">
					Merci pour votre message. Je vous répondrai dans les plus brefs délais.
				</p>
				<button
					on:click={resetSuccess}
					class="inline-flex items-center gap-2 rounded-md bg-blue-600 px-4 py-2 text-sm font-semibold text-white shadow-sm hover:bg-blue-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-blue-600 transition-all duration-200"
				>
					Envoyer un autre message
				</button>
			</div>
		{:else}
			<!-- Contact Form -->
			<form on:submit|preventDefault={handleSubmit} class="space-y-6">
				<!-- Name Field -->
				<div in:fly={{ y: 20, duration: 400, delay: 100 }}>
					<label for="name" class="block text-sm font-medium text-gray-700 mb-2">
						Nom complet *
					</label>
					<input
						type="text"
						id="name"
						bind:value={formData.name}
						on:input={(e) => handleInput('name', e.target.value)}
						on:blur={() => handleBlur('name')}
						class="block w-full rounded-md border-0 py-2 px-3 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-blue-600 sm:text-sm sm:leading-6 transition-colors duration-200"
						class:ring-red-500={errors.name && touched.name}
						class:focus:ring-red-600={errors.name && touched.name}
						placeholder="Votre nom complet"
						disabled={$contactLoading.isLoading}
					/>
					{#if errors.name && touched.name}
						<p class="mt-1 text-sm text-red-600">{errors.name}</p>
					{/if}
				</div>

				<!-- Email Field -->
				<div in:fly={{ y: 20, duration: 400, delay: 200 }}>
					<label for="email" class="block text-sm font-medium text-gray-700 mb-2">
						Adresse email *
					</label>
					<input
						type="email"
						id="email"
						bind:value={formData.email}
						on:input={(e) => handleInput('email', e.target.value)}
						on:blur={() => handleBlur('email')}
						class="block w-full rounded-md border-0 py-2 px-3 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-blue-600 sm:text-sm sm:leading-6 transition-colors duration-200"
						class:ring-red-500={errors.email && touched.email}
						class:focus:ring-red-600={errors.email && touched.email}
						placeholder="votre@email.com"
						disabled={$contactLoading.isLoading}
					/>
					{#if errors.email && touched.email}
						<p class="mt-1 text-sm text-red-600">{errors.email}</p>
					{/if}
				</div>

				<!-- Subject Field -->
				<div in:fly={{ y: 20, duration: 400, delay: 300 }}>
					<label for="subject" class="block text-sm font-medium text-gray-700 mb-2">
						Sujet *
					</label>
					<input
						type="text"
						id="subject"
						bind:value={formData.subject}
						on:input={(e) => handleInput('subject', e.target.value)}
						on:blur={() => handleBlur('subject')}
						class="block w-full rounded-md border-0 py-2 px-3 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-blue-600 sm:text-sm sm:leading-6 transition-colors duration-200"
						class:ring-red-500={errors.subject && touched.subject}
						class:focus:ring-red-600={errors.subject && touched.subject}
						placeholder="Sujet de votre message"
						disabled={$contactLoading.isLoading}
					/>
					{#if errors.subject && touched.subject}
						<p class="mt-1 text-sm text-red-600">{errors.subject}</p>
					{/if}
				</div>

				<!-- Message Field -->
				<div in:fly={{ y: 20, duration: 400, delay: 400 }}>
					<label for="message" class="block text-sm font-medium text-gray-700 mb-2">
						Message *
					</label>
					<textarea
						id="message"
						rows="6"
						bind:value={formData.message}
						on:input={(e) => handleInput('message', e.target.value)}
						on:blur={() => handleBlur('message')}
						class="block w-full rounded-md border-0 py-2 px-3 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-blue-600 sm:text-sm sm:leading-6 transition-colors duration-200 resize-vertical"
						class:ring-red-500={errors.message && touched.message}
						class:focus:ring-red-600={errors.message && touched.message}
						placeholder="Décrivez votre projet ou votre demande..."
						disabled={$contactLoading.isLoading}
					></textarea>
					<div class="mt-1 flex justify-between items-center">
						{#if errors.message && touched.message}
							<p class="text-sm text-red-600">{errors.message}</p>
						{:else}
							<div></div>
						{/if}
						<p class="text-xs text-gray-500" class:text-red-500={messageCharCount > messageCharLimit}>
							{messageCharCount}/{messageCharLimit}
						</p>
					</div>
				</div>

				<!-- Error messages are now handled by toast notifications -->

				<!-- Submit Button -->
				<div in:fly={{ y: 20, duration: 400, delay: 500 }}>
					<button
						type="submit"
						disabled={$contactLoading.isLoading || Object.keys(errors).length > 0}
						class="w-full flex justify-center items-center gap-2 rounded-md bg-blue-600 px-4 py-3 text-sm font-semibold text-white shadow-sm hover:bg-blue-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-blue-600 disabled:opacity-50 disabled:cursor-not-allowed transition-all duration-200 transform hover:scale-105 disabled:hover:scale-100"
					>
						{#if $contactLoading.isLoading}
							<LoadingSpinner size="sm" color="white" />
							Envoi en cours...
						{:else}
							<svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
								<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 19l9 2-9-18-9 18 9-2zm0 0v-8" />
							</svg>
							Envoyer le message
						{/if}
					</button>
				</div>

				<!-- Form Footer -->
				<div in:fade={{ duration: 400, delay: 600 }} class="text-center">
					<p class="text-xs text-gray-500">
						En envoyant ce message, vous acceptez que vos données soient utilisées pour vous répondre.
					</p>
				</div>
			</form>
		{/if}
	</div>
{/if}