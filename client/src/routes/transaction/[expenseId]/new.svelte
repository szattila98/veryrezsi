<script context="module" lang="ts">
	import type { Load } from '@sveltejs/kit';
	export const load: Load = async ({ session, params }) => {
		if (!session?.user) {
			return {
				status: 302,
				redirect: '/login',
			};
		}
		return {
			props: {
				userId: session.user.id,
				expenseId: params.expenseId,
			},
		};
	};
</script>

<script lang="ts">
	import Textfield, { HelperLine } from '@smui/textfield';
	import Button, { Label as ButtonLabel } from '@smui/button';
	import { form, field } from 'svelte-forms';
	import { required } from 'svelte-forms/validators';
	import { goto } from '$app/navigation';

	export let expenseId: number;

	const donorName = field('donorName', '', [required()]);
	let currencyType = { id: 0, abbreviation: 'HUF', name: 'base.currencies.huf' };
	const value = field('value', '', [required()]);
	const date = field('date', '', [required()]);
	const newTransactionForm = form(donorName, value, date);

	function newTransaction() {
		if (
			$newTransactionForm.valid &&
			$donorName.value &&
			currencyType &&
			$value.value &&
			$date.value
		) {
			return fetch('/api/transaction', {
				method: 'POST',
				body: JSON.stringify({
					donorName: $donorName.value,
					currencyType: currencyType,
					value: parseInt($value.value),
					date: $date.value,
					expenseId: expenseId,
				}),
				headers: {
					'Content-Type': 'application/json',
				},
			}).then(() => goto('/dashboard'));
		} else {
			newTransactionForm.validate();
		}
	}
</script>

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
				<HelperLine>Not a valid donor</HelperLine>
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
				<HelperLine>Not a valid donor</HelperLine>
			{/if}
		</svelte:fragment>
	</Textfield>

	<Button type="submit" on:click={newTransaction} variant="raised">
		<ButtonLabel>Login</ButtonLabel>
	</Button>
</div>

<style lang="scss">
	.form-container {
		display: flex;
		flex-direction: column;
		align-items: center;
	}

	* :global(.mdc-text-field) {
		width: 30%;
		margin: 1em 2em 0em;
	}

	* :global(.mdc-button) {
		margin: 1em 2em 0em;
	}

	* :global(.mdc-text-field-helper-line) {
		justify-content: center;
	}
</style>
