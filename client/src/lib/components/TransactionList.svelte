<script lang="ts">
	import type { Transaction } from '$mock/api/models/expense_model';

	import List, { Group, Subheader } from '@smui/list';
	import { createEventDispatcher } from 'svelte';
	import TransactionItem from './TransactionItem.svelte';

	export let transactions: Transaction[];
</script>

<div>
	{#if transactions.length > 0}
		<Group>
			<Subheader>Transactions</Subheader>
			<List class="transactionList">
				{#each transactions as transaction}
					<TransactionItem {transaction} on:deleteTransaction />
				{/each}
			</List>
		</Group>
	{:else}
		<p>No transactions added to this expense!</p>
	{/if}
</div>

<style>
	* :global(.transactionList) {
		border-left: 1px solid var(--mdc-theme-text-hint-on-background, rgba(0, 0, 0, 0.1));
		margin: 10px;
	}
</style>
