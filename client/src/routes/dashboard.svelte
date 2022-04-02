<script context="module" lang="ts">
	import type { Load } from '@sveltejs/kit';
	export const load: Load = async ({ session }) => {
		if (!session?.user) {
			return {
				status: 302,
				redirect: '/login',
			};
		}
		const res = await getExpenses({ userId: session.user.id });
		return {
			status: res.status,
			props: {
				expenses: res.data,
			},
		};
	};
</script>

<script lang="ts">
	import { Expense } from '$mock/api/models/expense_model';

	import Drawer, { AppContent, Content } from '@smui/drawer';
	import List, { Item, Text } from '@smui/list';
	import { getExpenses } from '$api/dashboard';
	import TransactionList from '$lib/components/TransactionList.svelte';
	import Separator from '@smui/list/src/Separator.svelte';
	import Button from '@smui/button/src/Button.svelte';

	export let expenses: Expense[] = [];

	let clickedExpense: Expense | null = expenses && expenses[0] ? expenses[0] : null;
	function onDrawerClick(expense: Expense) {
		clickedExpense = expense;
	}
</script>

<div class="drawer-container">
	<Drawer>
		<Content>
			<List>
				{#each expenses as expense, i (i)}
					<Item on:click={() => onDrawerClick(expense)}>
						<Text>{expense.name}</Text>
					</Item>
				{/each}
			</List>
		</Content>
	</Drawer>

	<AppContent class="app-content">
		<main class="main-content">
			{#if clickedExpense}
				<div class="expenseHeader">
					<h2>{clickedExpense.name}</h2>
					<Button>New Transaction</Button>
				</div>
				<pre>{clickedExpense.startDate}</pre>
				<p>{clickedExpense.description}</p>
				<p>
					{clickedExpense.recurrenceType.name} recurrence, {clickedExpense.recurrenceType.perYear}
					per year
				</p>
				<p>Cost: {clickedExpense.value} {clickedExpense.currencyType.abbreviation}</p>
				<Separator />
				<TransactionList transactions={clickedExpense.transactions} />
			{:else}
				<p>Nothing selected!</p>
			{/if}
		</main>
	</AppContent>
</div>

<style lang="scss">
	.drawer-container {
		position: relative;
		display: flex;
		height: 80vh;
		border: 1px solid var(--mdc-theme-text-hint-on-background, rgba(0, 0, 0, 0.1));
		overflow: hidden;
		z-index: 0;
	}

	* :global(.app-content) {
		flex: auto;
		overflow: auto;
		position: relative;
		flex-grow: 1;
	}

	.main-content {
		overflow: auto;
		padding: 16px;
		height: 100%;
		box-sizing: border-box;
	}

	.expenseHeader {
		display: flex;
		justify-content: space-between;
		align-items: center;
	}
</style>
