<script lang="ts">
	import Textfield, { HelperLine } from '@smui/textfield';
	import Button, { Label } from '@smui/button';
	import { form, field } from 'svelte-forms';
	import { required } from 'svelte-forms/validators';
	import Dialog, { Content, Title } from '@smui/dialog';
	import { NewTransaction } from '$mock/api/models/expense_model';
	import { createEventDispatcher } from 'svelte';

	export let expenseId: number;

	let open = false;

	const dispatch = createEventDispatcher<{ newTransaction: { transaction: NewTransaction } }>();

	const donorName = field('donorName', '', [required()]);
	let currencyType = { id: 0, abbreviation: 'HUF', name: 'base.currencies.huf' };
	const value = field('value', '', [required()]);
	const date = field('date', '', [required()]);
	const newTransactionForm = form(donorName, value, date);

	function onSubmit() {
		if (
			$newTransactionForm.valid &&
			$donorName.value &&
			currencyType &&
			$value.value &&
			$date.value
		) {
			dispatch('newTransaction', {
				transaction: {
					donorName: $donorName.value,
					currencyType: currencyType,
					value: parseInt($value.value),
					date: $date.value,
					expenseId: expenseId,
				},
			});
		} else {
			newTransactionForm.validate();
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

			<p>drop down placeholder</p>

			<Textfield
				variant="outlined"
				bind:value={$value.value}
				label="Value"
				bind:dirty={$value.dirty}
				bind:invalid={$value.invalid}
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

	* :global(.mdc-button) {
		margin: 1em 2em 0em;
		width: 100%;
	}
</style>
