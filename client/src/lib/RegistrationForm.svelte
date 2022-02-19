<script lang="ts">
	import LayoutGrid, { Cell } from '@smui/layout-grid';
	import Textfield, { HelperLine } from '@smui/textfield';
	import Button, { Label as ButtonLabel } from '@smui/button';
	import Snackbar, { Label as SnackbarLabel, SnackbarComponentDev } from '@smui/snackbar';
	import { form, field } from 'svelte-forms';
	import { required, email, between, matchField, pattern } from 'svelte-forms/validators';

	interface RegistrationForm {
		email: string;
		username: string;
		password: string;
	}

	// TODO other validators, write correct helper line errrors
	// TODO helper lines should go next to the text fields
	const mail = field('email', '', [required(), email()]);
	const username = field('username', '', [required(), between(6, 30)]);
	const password = field('password', '', [required(), between(8, 30)]);
	const confirmPassword = field('confirmPassword', '', [required(), matchField(password)]);
	const regForm = form(mail, username, password, confirmPassword);

	let failureSnackbar: SnackbarComponentDev;

	function register(rf: RegistrationForm) {
		if (
			!$regForm.valid ||
			!$mail.value ||
			!$username.value ||
			!$password.value ||
			!$confirmPassword.value
		) {
			failureSnackbar.close();
			failureSnackbar.open();
		} else {
			// TODO interaction with mock
			alert(JSON.stringify(rf));
		}
	}
</script>

<LayoutGrid>
	<Cell />
	<Cell>
		<div class="form-container">
			<Textfield
				variant="outlined"
				bind:value={$mail.value}
				label="Email address"
				bind:dirty={$mail.dirty}
				bind:invalid={$mail.invalid}
				><svelte:fragment slot="helper">
					{#if !$mail.valid}
						<HelperLine>Not a valid email address.</HelperLine>
					{/if}
				</svelte:fragment>
			</Textfield>

			<Textfield
				variant="outlined"
				bind:value={$username.value}
				label="Username"
				bind:dirty={$username.dirty}
				bind:invalid={$username.invalid}
				><svelte:fragment slot="helper">
					{#if !$username.valid}
						<HelperLine>Not a valid username.</HelperLine>
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
						<HelperLine>Not a valid password.</HelperLine>
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
						<HelperLine>Not the same as the password, or empty.</HelperLine>
					{/if}
				</svelte:fragment>
			</Textfield>

			<Button
				type="submit"
				on:click={() =>
					register({ email: $mail.value, username: $username.value, password: $password.value })}
				variant="raised"
			>
				<ButtonLabel>Register</ButtonLabel>
			</Button>
		</div>
	</Cell>
	<Cell />
</LayoutGrid>
<Snackbar bind:this={failureSnackbar}>
	<SnackbarLabel>Error: Please specify valid credentials!</SnackbarLabel>
</Snackbar>

<style lang="scss">
	.form-container {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: space-around;
		height: 300px;
	}
</style>
