<script lang="ts">
	import { goto } from '$app/navigation';

	import Textfield, { HelperLine } from '@smui/textfield';
	import Button, { Label as ButtonLabel } from '@smui/button';
	import Snackbar, { Label as SnackbarLabel, SnackbarComponentDev } from '@smui/snackbar';
	import { form, field } from 'svelte-forms';

	import { required } from 'svelte-forms/validators';
	import { user_api } from '../api/user';

	let failureSnackbar: SnackbarComponentDev;

	const user = field('user', '', [required()]);
	const password = field('password', '', [required()]);
	const loginForm = form(user, password);

	function login() {
		if ($loginForm.valid && $user.value && $password.value) {
			user_api
				.login({ user: $user.value, password: $password.value })
				.then((_res) => {
					goto('/home');
				})
				.catch((_err) => {
					openFailedLoginAlert();
				});
		} else {
			openFailedLoginAlert();
		}
	}

	function openFailedLoginAlert() {
		failureSnackbar.close();
		failureSnackbar.open();
	}
</script>

<div class="form-container">
	<Textfield
		variant="outlined"
		bind:value={$user.value}
		label="Username"
		bind:dirty={$user.dirty}
		bind:invalid={$user.invalid}
		><svelte:fragment slot="helper">
			{#if !$user.valid}
				<HelperLine>Not a valid username</HelperLine>
			{/if}
		</svelte:fragment>
	</Textfield>

	<Textfield
		type="password"
		variant="outlined"
		bind:value={$password.value}
		label="Password"
		bind:dirty={$password.dirty}
		bind:invalid={$password.invalid}
		><svelte:fragment slot="helper">
			{#if !$password.valid}
				<HelperLine>Not a valid password</HelperLine>
			{/if}
		</svelte:fragment>
	</Textfield>

	<Button type="submit" on:click={login} variant="raised">
		<ButtonLabel>Login</ButtonLabel>
	</Button>
</div>

<Snackbar bind:this={failureSnackbar}>
	<SnackbarLabel>Please specify valid credentials!</SnackbarLabel>
</Snackbar>

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
