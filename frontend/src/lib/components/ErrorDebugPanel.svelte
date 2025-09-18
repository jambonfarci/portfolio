<script lang="ts">
	import { onMount } from 'svelte';
	import { errorLogger, type LoggedError } from '$lib/utils/errorLogger';

	export let visible: boolean = false;

	let logs: LoggedError[] = [];
	let stats = { total: 0, byLevel: { error: 0, warning: 0, info: 0, debug: 0 }, recent: 0 };
	let selectedLevel: LoggedError['level'] | 'all' = 'all';

	onMount(() => {
		refreshLogs();
		
		// Refresh logs every 5 seconds
		const interval = setInterval(refreshLogs, 5000);
		
		return () => clearInterval(interval);
	});

	function refreshLogs() {
		logs = errorLogger.getLogs();
		stats = errorLogger.getStats();
	}

	function clearLogs() {
		errorLogger.clearLogs();
		refreshLogs();
	}

	function exportLogs() {
		const dataStr = errorLogger.exportLogs();
		const dataBlob = new Blob([dataStr], { type: 'application/json' });
		const url = URL.createObjectURL(dataBlob);
		const link = document.createElement('a');
		link.href = url;
		link.download = `error-logs-${new Date().toISOString().split('T')[0]}.json`;
		link.click();
		URL.revokeObjectURL(url);
	}

	function togglePanel() {
		visible = !visible;
	}

	$: filteredLogs = selectedLevel === 'all' 
		? logs 
		: logs.filter(log => log.level === selectedLevel);

	function formatTimestamp(date: Date): string {
		return new Date(date).toLocaleTimeString('fr-FR');
	}

	function getLevelColor(level: LoggedError['level']): string {
		switch (level) {
			case 'error': return 'text-red-600 bg-red-50';
			case 'warning': return 'text-yellow-600 bg-yellow-50';
			case 'info': return 'text-blue-600 bg-blue-50';
			case 'debug': return 'text-gray-600 bg-gray-50';
			default: return 'text-gray-600 bg-gray-50';
		}
	}
</script>

<!-- Debug Panel Toggle (only in development) -->
{#if import.meta.env.DEV}
	<button
		on:click={togglePanel}
		class="fixed bottom-4 right-4 z-50 bg-gray-800 text-white p-3 rounded-full shadow-lg hover:bg-gray-700 transition-colors duration-200"
		title="Ouvrir le panneau de débogage des erreurs"
	>
		<svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
			<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-2.5L13.732 4c-.77-.833-1.964-.833-2.732 0L3.732 16.5c-.77.833.192 2.5 1.732 2.5z" />
		</svg>
		{#if stats.total > 0}
			<span class="absolute -top-2 -right-2 bg-red-500 text-white text-xs rounded-full h-5 w-5 flex items-center justify-center">
				{stats.total}
			</span>
		{/if}
	</button>
{/if}

<!-- Debug Panel -->
{#if visible && import.meta.env.DEV}
	<div class="fixed inset-0 z-50 bg-black bg-opacity-50 flex items-center justify-center p-4">
		<div class="bg-white rounded-lg shadow-xl max-w-4xl w-full max-h-[90vh] overflow-hidden">
			<!-- Header -->
			<div class="bg-gray-800 text-white p-4 flex items-center justify-between">
				<h2 class="text-lg font-semibold">Panneau de débogage des erreurs</h2>
				<button
					on:click={togglePanel}
					class="text-gray-300 hover:text-white transition-colors duration-200"
				>
					<svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
						<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
					</svg>
				</button>
			</div>

			<!-- Stats -->
			<div class="p-4 bg-gray-50 border-b">
				<div class="grid grid-cols-2 md:grid-cols-5 gap-4 text-center">
					<div>
						<div class="text-2xl font-bold text-gray-900">{stats.total}</div>
						<div class="text-sm text-gray-600">Total</div>
					</div>
					<div>
						<div class="text-2xl font-bold text-red-600">{stats.byLevel.error}</div>
						<div class="text-sm text-gray-600">Erreurs</div>
					</div>
					<div>
						<div class="text-2xl font-bold text-yellow-600">{stats.byLevel.warning}</div>
						<div class="text-sm text-gray-600">Avertissements</div>
					</div>
					<div>
						<div class="text-2xl font-bold text-blue-600">{stats.byLevel.info}</div>
						<div class="text-sm text-gray-600">Infos</div>
					</div>
					<div>
						<div class="text-2xl font-bold text-gray-600">{stats.recent}</div>
						<div class="text-sm text-gray-600">Récents (1h)</div>
					</div>
				</div>
			</div>

			<!-- Controls -->
			<div class="p-4 border-b flex items-center justify-between gap-4">
				<div class="flex items-center gap-4">
					<select
						bind:value={selectedLevel}
						class="rounded-md border-gray-300 text-sm"
					>
						<option value="all">Tous les niveaux</option>
						<option value="error">Erreurs</option>
						<option value="warning">Avertissements</option>
						<option value="info">Infos</option>
						<option value="debug">Debug</option>
					</select>
					
					<button
						on:click={refreshLogs}
						class="px-3 py-1 bg-blue-600 text-white rounded text-sm hover:bg-blue-700 transition-colors duration-200"
					>
						Actualiser
					</button>
				</div>

				<div class="flex items-center gap-2">
					<button
						on:click={exportLogs}
						class="px-3 py-1 bg-green-600 text-white rounded text-sm hover:bg-green-700 transition-colors duration-200"
					>
						Exporter
					</button>
					<button
						on:click={clearLogs}
						class="px-3 py-1 bg-red-600 text-white rounded text-sm hover:bg-red-700 transition-colors duration-200"
					>
						Effacer
					</button>
				</div>
			</div>

			<!-- Logs List -->
			<div class="overflow-y-auto max-h-96">
				{#if filteredLogs.length === 0}
					<div class="p-8 text-center text-gray-500">
						<svg class="w-12 h-12 mx-auto mb-4 text-gray-300" fill="none" stroke="currentColor" viewBox="0 0 24 24">
							<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
						</svg>
						<p>Aucun log trouvé</p>
					</div>
				{:else}
					{#each filteredLogs.reverse() as log, index}
						<div class="border-b border-gray-200 p-4 hover:bg-gray-50">
							<div class="flex items-start justify-between gap-4">
								<div class="flex-1">
									<div class="flex items-center gap-2 mb-2">
										<span class="px-2 py-1 rounded text-xs font-medium {getLevelColor(log.level)}">
											{log.level.toUpperCase()}
										</span>
										<span class="text-sm text-gray-500">
											{formatTimestamp(log.timestamp)}
										</span>
										{#if log.context?.component}
											<span class="text-xs text-gray-400">
												{log.context.component}
											</span>
										{/if}
									</div>
									
									<div class="text-sm text-gray-900 mb-2">
										{log.message}
									</div>
									
									{#if log.context?.action}
										<div class="text-xs text-gray-600 mb-1">
											Action: {log.context.action}
										</div>
									{/if}
									
									{#if log.stack}
										<details class="mt-2">
											<summary class="text-xs text-gray-500 cursor-pointer hover:text-gray-700">
												Voir la stack trace
											</summary>
											<pre class="mt-2 text-xs bg-gray-100 p-2 rounded overflow-x-auto">{log.stack}</pre>
										</details>
									{/if}
								</div>
							</div>
						</div>
					{/each}
				{/if}
			</div>
		</div>
	</div>
{/if}