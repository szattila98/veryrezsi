<script lang="ts">
	import type { SnackbarComponentDev } from '@smui/snackbar';

	import Textfield, { HelperLine } from '@smui/textfield';
	import Button, { Label as ButtonLabel } from '@smui/button';
	import Snackbar, { Label as SnackbarLabel } from '@smui/snackbar';
	import { form, field } from 'svelte-forms';

	import { required } from 'svelte-forms/validators';
	import type { LoginRequestData } from '$shared/api/login';

	let failureSnackbar: SnackbarComponentDev;

	const user = field('user', '', [required()]);
	const password = field('password', '', [required()]);
	const loginForm = form(user, password);

	function login() {
		if ($loginForm.valid && $user.value && $password.value) {
			handleLogin({ user: $user.value, password: $password.value })
				.then((res) => {
					if (!res.ok) {
						openFailedLoginAlert();
						return;
					}

					// Just a hack to use instead goto(), goto refused to load session in some cases
					// TODO
					window.location.href = '/';
				})
				.catch(() => {
					openFailedLoginAlert();
					return;
				});
		} else {
			openFailedLoginAlert();
		}
	}

	function handleLogin(data: LoginRequestData) {
		return fetch('/api/login', {
			method: 'POST',
			body: JSON.stringify(data),
			headers: {
				'Content-Type': 'application/json',
			},
		});
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

	<a href="/register">Sign me up.</a>
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
