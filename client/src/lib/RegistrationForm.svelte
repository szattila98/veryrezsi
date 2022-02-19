<script lang="ts">
	import LayoutGrid, { Cell } from '@smui/layout-grid';
	import Textfield from '@smui/textfield';
	import Button, { Label } from '@smui/button';
	import { form, field } from 'svelte-forms';
	import { required, email, between, matchField, pattern } from 'svelte-forms/validators';

	interface RegistrationForm {
		email: string;
		username: string;
		password: string;
	}

	const mail = field('email', '', [required(), email()]);
	const username = field('username', '', [required(), between(6, 30)]);
	const password = field('password', '', [required(), between(8, 30)]);
	const confirmPassword = field('confirmPassword', '', [required(), matchField(password)]);
	const regForm = form(mail, username, password, confirmPassword);

	function register(rf: RegistrationForm) {
		if (!$mail.value && !$username.value && !$password.value && !$confirmPassword.value) {
			alert('Type your info please!'); // TODO replace alert with toast or smth
		}
		// TODO interaction with mock
	}
</script>

<LayoutGrid>
	<Cell />
	<Cell>
		<div class="form-container">
			<Textfield variant="outlined" bind:value={$mail.value} label="Email" />
			<!-- TODO render helper line conditionally -->
			<Textfield variant="outlined" bind:value={$username.value} label="Username" />
			<Textfield variant="outlined" bind:value={$password.value} label="Password" />
			<Textfield variant="outlined" bind:value={$confirmPassword.value} label="Confirm Password" />
			<Button
				type="submit"
				on:click={() =>
					register({ email: $mail.value, username: $username.value, password: $password.value })}
				variant="raised"
				disabled={$regForm.dirty}
			>
				<Label>Register</Label>
			</Button>
		</div>
	</Cell>
	<Cell />
</LayoutGrid>

<style lang="scss">
	.form-container {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: space-around;
		height: 300px;
	}
</style>
