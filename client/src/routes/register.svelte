<script lang="ts">
	import type { RegisterRequestData } from '$mock/api/models/register_model';

	import Textfield, { HelperLine } from '@smui/textfield';
	import Button, { Label as ButtonLabel } from '@smui/button';
	import Snackbar, { Label as SnackbarLabel } from '@smui/snackbar';
	import type { SnackbarComponentDev } from '@smui/snackbar';
	import { form, field } from 'svelte-forms';
	import { email, between, matchField, pattern } from 'svelte-forms/validators';

	import { PASSWORD_REGEXP } from '$lib/const';

	let failureSnackbar: SnackbarComponentDev;

	const mail = field('email', '', [email()]);
	const user = field('user', '', [between(8, 30)]);
	const password = field('password', '', [pattern(PASSWORD_REGEXP)]);
	const confirmPassword = field('confirmPassword', '', [matchField(password)]);
	const regForm = form(mail, user, password, confirmPassword);

	function register() {
		if ($regForm.valid && $mail.value && $user.value && $password.value && $confirmPassword.value) {
			handleRegister({ email: $mail.value, user: $user.value, password: $password.value }).then(
				(res) => {
					if (!res.ok) {
						openFailedRegisterAlert();
					}

					// Just a hack to use instead goto(), goto refused to load session in some cases
					// TODO
					window.location.href = '/';
				}
			);
		} else {
			openFailedRegisterAlert();
		}
	}

	async function handleRegister(data: RegisterRequestData) {
		return fetch('/api/register', {
			method: 'POST',
			body: JSON.stringify(data),
			headers: {
				'Content-Type': 'application/json',
			},
		});
	}

	function openFailedRegisterAlert() {
		failureSnackbar.close();
		failureSnackbar.open();
	}
</script>

<div class="form-container">
	<Textfield
		variant="outlined"
		bind:value={$mail.value}
		label="Email address"
		bind:dirty={$mail.dirty}
		bind:invalid={$mail.invalid}
		class="form-field"
		><svelte:fragment slot="helper">
			{#if !$mail.valid}
				<dev class="helper-text">
					<HelperLine>Not a valid email address</HelperLine>
				</dev>
			{/if}
		</svelte:fragment>
	</Textfield>

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

	<Textfield
		type="password"
		variant="outlined"
		bind:value={$confirmPassword.value}
		label="Confirm Password"
		bind:dirty={$confirmPassword.dirty}
		bind:invalid={$confirmPassword.invalid}
		><svelte:fragment slot="helper">
			{#if !$confirmPassword.valid}
				<HelperLine>Not the same as the password</HelperLine>
			{/if}
		</svelte:fragment>
	</Textfield>

	<Button type="submit" on:click={register} variant="raised">
		<ButtonLabel>Register</ButtonLabel>
	</Button>
	<a href="/login">Log me in.</a>
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
