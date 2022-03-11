<script context="module">
	export async function load({ session, fetch }) {
		if (!session?.user) {
			return {
				status: 302,
				redirect: '/login',
			};
		}
		const res = await getExpenses(session.user.id);
		return {
			status: res.status,
			props: {
				expenses: res.data,
			},
		};
	}
</script>

<script lang="ts">
	import { Expense } from '$mock/api/models/expense_model';

	import Drawer, { AppContent, Content } from '@smui/drawer';
	import List, { Item, Text } from '@smui/list';
	import { getExpenses } from './api/dashboard';

	export let expenses: Expense[] = [];
	let clickedExpense: Expense | null = null;
	// is this allowed? im not sure
	if (clickedExpense === null && expenses) {
		clickedExpense = expenses[0];
	}

	function onDrawerClick(expense: Expense) {
		clickedExpense = expense;
	}
</script>

<div class="drawer-container">
	<Drawer>
		<Content>
			<List>
				{#each expenses as expense, i (i)}
					<Item href="javascript:void(0)" on:click={() => onDrawerClick(expense)}>
						<Text>{expense.name}</Text>
					</Item>
				{/each}
			</List>
		</Content>
	</Drawer>

	<AppContent class="app-content">
		<main class="main-content">
			{#if clickedExpense}
				<h2>{clickedExpense.name}</h2>
				<pre>{clickedExpense.startDate}</pre>
				<p>{clickedExpense.description}</p>
				<p>Cost: {clickedExpense.value}</p>
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
</style>
