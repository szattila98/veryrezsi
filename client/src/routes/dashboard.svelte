<script context="module" lang="ts">
	import type { Load } from '@sveltejs/kit';
	export const load: Load = async ({ session }) => {
		if (!session?.user) {
			return {
				status: 302,
				redirect: '/login',
			};
		}
		const [predefinedExpensesResponse, currencyTypesResponse, recurrenceTypesResponse] =
			await Promise.all([getPredefinedExpenses(), getCurrencyTypes(), getRecurrenceTypes()]);
		return {
			props: {
				userId: session.user.id,
				predefinedExpenses: predefinedExpensesResponse.data,
				currencyTypes: currencyTypesResponse.data,
				recurrenceTypes: recurrenceTypesResponse.data,
			},
		};
	};
</script>

<script lang="ts">
	import { onMount } from 'svelte';

	import Drawer, { AppContent, Content } from '@smui/drawer';
	import List, { Item, Text } from '@smui/list';
	import TransactionList from '$lib/components/TransactionList.svelte';
	import Separator from '@smui/list/src/Separator.svelte';
	import Button from '@smui/button/src/Button.svelte';
	import NewExpenseDialog from '$lib/components/NewExpenseDialog.svelte';
	import NewTransactionDialog from '$lib/components/NewTransactionDialog.svelte';
	import { getCurrencyTypes } from './api/currency';
	import { getRecurrenceTypes } from './api/recurrence';
	import { getPredefinedExpenses } from './api/expense';
	import type { CurrencyType, Expense, PredefinedExpense, RecurrenceType } from '$shared/domain';
	import type { NewExpense } from '$shared/api/newExpense';
	import type { NewTransaction } from '$shared/api/newTransaction';

	export let userId: number;
	export let predefinedExpenses: PredefinedExpense[] = [];
	export let recurrenceTypes: RecurrenceType[] = [];
	export let currencyTypes: CurrencyType[] = [];

	let expenses: Expense[] = [];
	let clickedExpense: Expense | null;
	let newExpenseDialog: NewExpenseDialog;
	let newTransactionDialog: NewTransactionDialog;

	onMount(async () => {
		const res = await fetch(`/api/expense/${userId}`, {
			method: 'GET',
			headers: {
				'Content-Type': 'application/json',
			},
		});
		const data = (await res.json()).data;
		expenses = data.expenses as Expense[];
		clickedExpense = expenses && expenses[0] ? expenses[0] : null;
	});

	$: clickedExpense = clickedExpense
		? expenses.filter((expense) => {
				return expense.id === clickedExpense?.id;
		  })[0] || null
		: null;

	function onDrawerClick(expense: Expense) {
		clickedExpense = expense;
	}

	function newExpenseHandle(event: CustomEvent<{ expense: NewExpense }>) {
		fetch('/api/expense', {
			method: 'POST',
			body: JSON.stringify(event.detail.expense),
			headers: {
				'Content-Type': 'application/json',
			},
		}).then(() => location.reload());
	}

	function newTransactionHandle(event: CustomEvent<{ transaction: NewTransaction }>) {
		fetch('/api/transaction', {
			method: 'POST',
			body: JSON.stringify(event.detail.transaction),
			headers: {
				'Content-Type': 'application/json',
			},
		}).then(() => location.reload());
	}

	function deleteTransactionHandle(event: CustomEvent<{ transactionId: number }>) {
		fetch(`/api/transaction/${event.detail.transactionId}/delete`, {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json',
			},
		}).then(() => {
			fetch(`/api/expense/${userId}`, {
				method: 'GET',
				headers: {
					'Content-Type': 'application/json',
				},
			}).then((res) => {
				res.json().then((res) => {
					expenses = res.data.expenses;
				});
			});
		});
	}
</script>

<div class="drawer-container">
	<Drawer>
		<Button on:click={newExpenseDialog.show}>New Expense</Button>
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
					<Button on:click={newTransactionDialog.show}>New Transaction</Button>
				</div>
				<pre>{clickedExpense.startDate}</pre>
				<p>{clickedExpense.description}</p>
				<p>
					{clickedExpense.recurrenceType.name} recurrence, {clickedExpense.recurrenceType.perYear}
					per year
				</p>
				<p>Cost: {clickedExpense.value} {clickedExpense.currencyType.abbreviation}</p>
				<Separator />
				<TransactionList
					transactions={clickedExpense.transactions}
					on:deleteTransaction={deleteTransactionHandle}
				/>
			{:else}
				<p>Nothing selected!</p>
			{/if}
		</main>
	</AppContent>
</div>

<NewExpenseDialog
	{userId}
	{predefinedExpenses}
	{currencyTypes}
	{recurrenceTypes}
	on:newExpense={newExpenseHandle}
	bind:this={newExpenseDialog}
/>

{#if clickedExpense}
	<NewTransactionDialog
		expenseId={clickedExpense.id}
		{currencyTypes}
		on:newTransaction={newTransactionHandle}
		bind:this={newTransactionDialog}
	/>
{/if}

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
