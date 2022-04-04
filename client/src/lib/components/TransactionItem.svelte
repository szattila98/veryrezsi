<script lang="ts">
	import { Transaction } from '$mock/api/models/expense_model';
	import IconButton from '@smui/icon-button';

	import { Item, Text, PrimaryText, SecondaryText } from '@smui/list';
	import { createEventDispatcher } from 'svelte';

	export let transaction: Transaction;

	const dispatch = createEventDispatcher<{ deleteTransaction: { transactionId: number } }>();
</script>

<Item style="width: 100%; display: flex; justify-content: space-between; align-items: center;">
	<Text>
		<PrimaryText>From {transaction.donorName} - {transaction.date}</PrimaryText>
		<SecondaryText>{transaction.value} {transaction.currencyType.abbreviation}</SecondaryText>
	</Text>
	<IconButton
		class="material-icons"
		on:click={() => dispatch('deleteTransaction', { transactionId: transaction.id })}
		>close</IconButton
	>
</Item>

<style lang="scss"></style>
