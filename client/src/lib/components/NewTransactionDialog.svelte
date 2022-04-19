<script lang="ts">
	import Textfield, { HelperLine } from '@smui/textfield';
	import Button, { Label } from '@smui/button';
	import { form, field } from 'svelte-forms';
	import { required } from 'svelte-forms/validators';
	import Dialog, { Content, Title } from '@smui/dialog';
	import { CurrencyType, NewTransaction } from '$mock/api/models/expense_model';
	import { createEventDispatcher } from 'svelte';
	import Select, { Option } from '@smui/select';

	export let expenseId: number;
	export let currencyTypes: CurrencyType[];

	let open = false;

	const dispatch = createEventDispatcher<{ newTransaction: { transaction: NewTransaction } }>();

	const donorName = field('donorName', '', [required()]);
	const selectedCurrencyTypeId = field('selectedCurrencyTypeId', 0, [required()]);
	const value = field('value', 0, [required()]);
	const date = field('date', '', [required()]);
	const newTransactionForm = form(donorName, value, date);

	async function onSubmit() {
		await newTransactionForm.validate();
		if ($newTransactionForm.valid) {
			console.log('nein');
			dispatch('newTransaction', {
				transaction: {
					donorName: $donorName.value,
					currencyTypeId: $selectedCurrencyTypeId.value,
					value: $value.value,
					date: $date.value,
					expenseId: expenseId,
				},
			});
		}
	}

	export function show() {
		open = true;
	}
</script>

<Dialog bind:open aria-labelledby="title" aria-describedby="content" actions={null}>
	<Title id="title">New Transaction</Title>
	<Content id="content">
		<div class="form-container">
			<Textfield
				variant="outlined"
				bind:value={$donorName.value}
				label="Donor Name"
				bind:dirty={$donorName.dirty}
				bind:invalid={$donorName.invalid}
				><svelte:fragment slot="helper">
					{#if !$donorName.valid}
						<HelperLine>Not a valid donor</HelperLine>
					{/if}
				</svelte:fragment>
			</Textfield>

			<Select bind:value={$selectedCurrencyTypeId.value} label="Currency">
				{#each currencyTypes as currency}
					<Option value={currency.id}>{currency.abbreviation}</Option>
				{/each}
				<svelte:fragment slot="helperText">
					{#if !$selectedCurrencyTypeId.valid}
						<HelperLine>Not a valid currency</HelperLine>
					{/if}
				</svelte:fragment>
			</Select>

			<Textfield
				variant="outlined"
				bind:value={$value.value}
				label="Value"
				bind:dirty={$value.dirty}
				bind:invalid={$value.invalid}
				type="number"
				><svelte:fragment slot="helper">
					{#if !$value.valid}
						<HelperLine>Not a valid value</HelperLine>
					{/if}
				</svelte:fragment>
			</Textfield>

			<Textfield
				variant="outlined"
				bind:value={$date.value}
				label="Date"
				bind:dirty={$date.dirty}
				bind:invalid={$date.invalid}
				><svelte:fragment slot="helper">
					{#if !$date.valid}
						<HelperLine>Not a valid date</HelperLine>
					{/if}
				</svelte:fragment>
			</Textfield>

			<Button type="submit" on:click={onSubmit} variant="raised">
				<Label>Insert</Label>
			</Button>
		</div>
	</Content>
</Dialog>

<style lang="scss">
	.form-container {
		display: flex;
		flex-direction: column;
		align-items: center;
	}

	* :global(.mdc-text-field) {
		margin: 1em 2em 0em;
		width: 100%;
	}

	* :global(.mdc-text-field-helper-line) {
		justify-content: center;
	}

	* :global(.mdc-select) {
		margin: 1em 2em 0em;
		width: 100%;
	}

	* :global(.mdc-button) {
		margin: 1em 2em 0em;
		width: 100%;
	}
</style>
