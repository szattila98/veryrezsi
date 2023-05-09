<script lang="ts">
	import type { LayoutServerData } from '../$types';
	import type { AutocompleteOption } from '@skeletonlabs/skeleton';
	import { Autocomplete } from '@skeletonlabs/skeleton';

	export let data: LayoutServerData;

	let selectedPredefinedExpenseLabel = "";

	const mockPredefinedExpenses: AutocompleteOption[] = [
		{ label: 'Netflix - Basic', value: 'netflixbasic', keywords: 'streaming, movies, series', meta: { healthy: false } },
		{ label: 'Netflix - Standard', value: 'netflixstandard', keywords: 'streaming, movies, series', meta: { healthy: false } },
		{ label: 'Netflix - Premium', value: 'netflixpremium', keywords: 'streaming, movies, series', meta: { healthy: true } },
		{ label: 'Synology C2 Backup - 100 Gb', value: 'synologybackup100', keywords: 'backup', meta: { healthy: false } },
	];

	function onPredefinedExpenseSelection(event: any): void {
		selectedPredefinedExpenseLabel = event.detail.label;
	}
</script>

<div>
	<h1 class="pt-2 md:pt-20 text-center text-4xl text-surface-50 font-semibold">
		Track your planned payments with us.<img class="inline h-20" alt="Stars emoji" src="/stars-emoji.png"/>
	</h1>
	<div class="flex flex-wrap justify-center text-center items-center mt-16 mx-8">
		<div class="basis-1/3 flex-1">
			<div class="card">
				<header class="card-header font-semibold">Use our curated list of subscriptions</header>
				<section class="p-4 h-64">
					<input
						class="input"
						type="search"
						name="predefinedExpense"
						bind:value={selectedPredefinedExpenseLabel}
						placeholder="Select the type of your subscription"
					/>
					<div class="w-full max-h-48 p-4 overflow-y-auto">
						<Autocomplete
							bind:input={selectedPredefinedExpenseLabel}
							options={mockPredefinedExpenses}
							on:selection={onPredefinedExpenseSelection}
						/>
					</div>
				</section>
			</div>
		</div>
		<div class="basis-1/5 flex-1">
			<span class="badge variant-filled p-4 text-sm">Or create a brand new</span>
		</div>
		<div class="basis-1/3 flex-1">asd</div>
	</div>
</div>

