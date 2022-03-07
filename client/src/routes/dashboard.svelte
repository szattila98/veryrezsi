<script lang="ts">
	import Drawer, { AppContent, Content } from '@smui/drawer';
	import List, { Item, Text } from '@smui/list';
	import { Expense, expense_api } from '../api/expenses';

	let expenses = expense_api.listAllExpenses(1);
	let clickedExpense: Expense | null = null;

	function onDrawerClick(expense: Expense) {
		clickedExpense = expense;
	}
</script>

{#await expenses}
	<p>Loading expenses</p>
{:then items}
	<div class="drawer-container">
		<Drawer>
			<Content>
				<List>
					{#each items as item, i (item.id)}
						<Item href="javascript:void(0)" on:click={() => onDrawerClick(item)}>
							<Text>{item.name}</Text>
						</Item>
					{/each}
				</List>
			</Content>
		</Drawer>

		<AppContent class="app-content">
			<main>
				{#if clickedExpense}
					<h2>{clickedExpense.name}</h2>
					<pre>{clickedExpense.startDate}</pre>
					<p>{clickedExpense.description}</p>
					<p>Cost: {clickedExpense.value}</p>
				{:else}
					Nothing selected!
				{/if}
			</main>
		</AppContent>
	</div>
{:catch error}
	<p>Something went wrong: {error.message}</p>
{/await}

<style lang="scss">
	.drawer-container {
		display: flex;
		border: 1px solid var(--mdc-theme-text-hint-on-background, rgba(0, 0, 0, 0.1));
	}

	* :global(.app-content) {
		display: flex;
		flex: auto;
		overflow: auto;
		position: relative;
		flex-grow: 1;
		padding: 16px;
	}
</style>
