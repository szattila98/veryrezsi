<script lang="ts">
	import type { LayoutServerData } from '../$types';
	import type { AutocompleteOption } from '@skeletonlabs/skeleton';
	import { Autocomplete } from '@skeletonlabs/skeleton';

	export let data: LayoutServerData;

	let selectedPredefinedExpenseLabel = '';

	const mockPredefinedExpenses: AutocompleteOption[] = [
		{
			label: 'Netflix - Basic',
			value: 'netflixbasic',
			keywords: 'streaming, movies, series',
			meta: { healthy: false }
		},
		{
			label: 'Netflix - Standard',
			value: 'netflixstandard',
			keywords: 'streaming, movies, series',
			meta: { healthy: false }
		},
		{
			label: 'Netflix - Premium',
			value: 'netflixpremium',
			keywords: 'streaming, movies, series',
			meta: { healthy: true }
		},
		{
			label: 'Synology C2 Backup - 100 Gb',
			value: 'synologybackup100',
			keywords: 'backup',
			meta: { healthy: false }
		}
	];

	function onPredefinedExpenseSelection(event: any): void {
		selectedPredefinedExpenseLabel = event.detail.label;
	}
</script>

<div>
	<h1 class="pt-2 text-center text-4xl font-semibold text-surface-50 md:pt-20">
		Track your planned payments with us.<img
			class="inline h-20"
			alt="Stars emoji"
			src="/stars-emoji.png"
		/>
	</h1>
	<div class="mx-8 mt-16 flex flex-wrap items-center justify-center text-center">
		<div class="flex-1 basis-1/3">
			<div class="card">
				<header class="card-header font-semibold">Use our curated list of subscriptions</header>
				<section class="h-64 p-4">
					<input
						class="input"
						type="search"
						name="predefinedExpense"
						bind:value={selectedPredefinedExpenseLabel}
						placeholder="Select the type of your subscription"
					/>
					<div class="max-h-48 w-full overflow-y-auto p-4">
						<Autocomplete
							bind:input={selectedPredefinedExpenseLabel}
							options={mockPredefinedExpenses}
							on:selection={onPredefinedExpenseSelection}
						/>
					</div>
				</section>
			</div>
		</div>
		<div class="flex-1 basis-1/5">
			<span class="badge variant-filled p-4 text-sm">Or create a brand new</span>
		</div>
		<div class="flex-1 basis-1/3">asd</div>
	</div>
</div>
