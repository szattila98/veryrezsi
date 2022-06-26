<script lang="ts">
	import type {
		PredefinedExpense,
		CurrencyType,
		RecurrenceType,
		NewExpense,
	} from '$mock/api/models/expense_model';

	import Textfield, { HelperLine } from '@smui/textfield';
	import Button, { Label } from '@smui/button';
	import { form, field } from 'svelte-forms';
	import { required } from 'svelte-forms/validators';
	import Dialog, { Content, Title } from '@smui/dialog';
	import { createEventDispatcher } from 'svelte';
	import Select, { Option } from '@smui/select';

	export let predefinedExpenses: PredefinedExpense[];
	export let currencyTypes: CurrencyType[];
	export let recurrenceTypes: RecurrenceType[];

	let open = false;

	const dispatch = createEventDispatcher<{ newExpense: { expense: NewExpense } }>();

	const newExpenseForm = form();

	async function onSubmit() {
		await newExpenseForm.validate();
		if ($newExpenseForm.valid) {
			//dispatch('newExpense', {
			//	expense: {},
			//});
		}
	}

	export function show() {
		open = true;
	}
</script>

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
