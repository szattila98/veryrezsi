<script lang="ts">
	import Textfield, { HelperLine } from '@smui/textfield';
	import Button, { Label } from '@smui/button';
	import { form, field } from 'svelte-forms';
	import { required } from 'svelte-forms/validators';
	import Dialog, { Content, Title } from '@smui/dialog';
	import { createEventDispatcher } from 'svelte';
	import Select, { Option } from '@smui/select';
	import type { CurrencyType, PredefinedExpense, RecurrenceType } from '$shared/domain';
	import type { NewExpense } from '$shared/api/newExpense';

	export let userId: number; // TODO field not needed with actual server
	export let predefinedExpenses: PredefinedExpense[];
	export let currencyTypes: CurrencyType[];
	export let recurrenceTypes: RecurrenceType[];

	let open = false;

	const dispatch = createEventDispatcher<{ newExpense: { expense: NewExpense } }>();

	let predefinedExpenseId: number | null = null;
	const name = field('name', '', [required()]);
	const description = field('description', '', [required()]);
	const startDate = field('startDate', '', [required()]);
	const value = field('value', '', [required()]);
	const currencyTypeId = field('currencyTypeId', 0, [required()]);
	const recurrenceTypeId = field('recurrenceTypeId', 0, [required()]);
	const newExpenseForm = form(
		name,
		description,
		startDate,
		value,
		currencyTypeId,
		recurrenceTypeId
	);

	async function onSubmit() {
		await newExpenseForm.validate();
		if ($newExpenseForm.valid) {
			dispatch('newExpense', {
				expense: {
					name: $name.value,
					description: $description.value,
					predefinedExpenseId,
					startDate: $startDate.value,
					value: $value.value,
					currencyTypeId: $currencyTypeId.value,
					recurrenceTypeId: $recurrenceTypeId.value,
					userId,
				},
			});
		}
	}

	function onPredefinedExpenseSelect(id: number | null) {
		const predefinedExpense = predefinedExpenses.find((expense) => id === expense.id);
		if (predefinedExpense) {
			predefinedExpenseId = predefinedExpense.id;
			$name.value = predefinedExpense.name;
			$description.value = predefinedExpense.description;
			$value.value = predefinedExpense.value + ''; // TODO solve discrepancy between predefined expense number value and expense string value
			$currencyTypeId.value = predefinedExpense.currencyType.id;
			$recurrenceTypeId.value = predefinedExpense.recurrenceType.id;
		} else {
			predefinedExpenseId = null;
			$name.value = '';
			$description.value = '';
			$value.value = '';
			$currencyTypeId.value = 0;
			$recurrenceTypeId.value = 0;
		}
	}

	export function show() {
		open = true;
	}
</script>

<Dialog bind:open aria-labelledby="title" aria-describedby="content" actions={null}>
	<Title id="title">New Expense</Title>
	<Content id="content">
		<div class="form-container">
			<Select
				bind:value={predefinedExpenseId}
				label="Predefined expense"
				on:SMUIMenu:selected={() => onPredefinedExpenseSelect(predefinedExpenseId)}
			>
				<Option value={-1}>-</Option>
				{#each predefinedExpenses as predefinedExpense}
					<Option value={predefinedExpense.id}>{predefinedExpense.name}</Option>
				{/each}
			</Select>

			<Textfield
				variant="outlined"
				bind:value={$name.value}
				label="Name"
				bind:dirty={$name.dirty}
				bind:invalid={$name.invalid}
				><svelte:fragment slot="helper">
					{#if !$name.valid}
						<HelperLine>Please enter the expense name</HelperLine>
					{/if}
				</svelte:fragment>
			</Textfield>

			<Textfield
				textarea
				variant="outlined"
				bind:value={$description.value}
				label="Description"
				bind:dirty={$description.dirty}
				bind:invalid={$description.invalid}
				input$resizable={false}
				><svelte:fragment slot="helper">
					{#if !$name.valid}
						<HelperLine>Please enter a description</HelperLine>
					{/if}
				</svelte:fragment>
			</Textfield>

			<Textfield
				variant="outlined"
				bind:value={$startDate.value}
				label="Date"
				bind:dirty={$startDate.dirty}
				bind:invalid={$startDate.invalid}
				><svelte:fragment slot="helper">
					{#if !$startDate.valid}
						<HelperLine>Please enter the expense start date</HelperLine>
					{/if}
				</svelte:fragment>
			</Textfield>

			<Textfield
				variant="outlined"
				bind:value={$value.value}
				label="Value"
				bind:dirty={$value.dirty}
				bind:invalid={$value.invalid}
				type="number"
				><svelte:fragment slot="helper">
					{#if !$value.valid}
						<HelperLine>Please enter the expense value</HelperLine>
					{/if}
				</svelte:fragment>
			</Textfield>

			<Select bind:value={$currencyTypeId.value} label="Currency">
				{#each currencyTypes as currency}
					<Option value={currency.id}>{currency.abbreviation}</Option>
				{/each}
				<svelte:fragment slot="helperText">
					{#if !$currencyTypeId.valid}
						<HelperLine>Please select a currency</HelperLine>
					{/if}
				</svelte:fragment>
			</Select>

			<Select bind:value={$recurrenceTypeId.value} label="Recurrence">
				{#each recurrenceTypes as recurrence}
					<Option value={recurrence.id}>{recurrence.name}</Option>
				{/each}
				<svelte:fragment slot="helperText">
					{#if !$recurrenceTypeId.valid}
						<HelperLine>Please select a recurrence</HelperLine>
					{/if}
				</svelte:fragment>
			</Select>

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
