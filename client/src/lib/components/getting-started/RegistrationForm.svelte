<script lang="ts">
	import { createEventDispatcher } from 'svelte';
	import type { RegisterRequestData } from '$shared/api/register';
	import type { Validator } from 'svelte-use-form';
	import { useForm, Hint, validators, required, email, HintGroup, pattern } from 'svelte-use-form';
	import {
		REQUIRED_VIOLATION_MSG,
		EMAIL_VIOLATION_MSG,
		TECHNICAL_ERROR_ALERT_MSG
	} from '$shared/constants';
	import AlertMsg from '../common/AlertMsg.svelte';
	import ThemedValidationHint from '../common/ThemedValidationHint.svelte';
	import { createFormState, type BaseFormStates } from '$shared/composables/createFormState';

	const dispatch = createEventDispatcher<{ switchView: void }>();
	const STRONG_PASSWORD_PATTERN =
		/^(?=.*[a-z])(?=.*[A-Z])(?=.*\d)(?=.*[@$!%*?&])[A-Za-z\d@$!%*?&]{8,120}$/;

	const form = useForm({ email: {}, username: {}, password: {}, confirmPassword: {} });

	const userInfo: RegisterRequestData = {
		email: '',
		username: '',
		password: '',
		confirmPassword: ''
	};

	const { formState, setFormState } = createFormState<BaseFormStates>();

	const passwordMatch: Validator = (value, form) => {
		return value === form.password?.value ? null : { passwordMatch: 'Passwords are not matching' };
	};

	async function register() {
		$form.touched = true;
		if ($form.valid) {
			try {
				await callRegisterApi(userInfo);
			} catch (err) {
				console.error('Registration error', err);
			}
		}
	}

	async function callRegisterApi(userInfo: RegisterRequestData) {
		try {
			const res = await fetch('/api/user/register', {
				method: 'POST',
				body: JSON.stringify(userInfo)
			});
			if (res.ok) {
				return navigateToLogin();
			} else {
				throw new Error('Registration unsuccessful');
			}
		} catch (err) {
			setFormState('TECHNICAL_ERROR');
			if (err instanceof Error) {
				throw new Error(`Sorry, you need to wait until we fix this ${err.message}`);
			}
		}
	}

	function navigateToLogin() {
		dispatch('switchView');
	}

	function validateConfirmPassword() {
		userInfo.confirmPassword &&
			!$form.confirmPassword.validate() &&
			($form.confirmPassword.touched = true);
	}
</script>

<svelte:head>
	<title>Register to VeryRezsi</title>
	<meta name="robots" content="noindex, nofollow" />
</svelte:head>

<form
	class="card variant-filled-surface p-8"
	id="register"
	autocomplete="on"
	novalidate
	use:form
	on:submit|preventDefault={register}
>
	<div class="mb-4">
		<label class="label" for="email">Username</label>
		<input
			class="input"
			id="username"
			name="username"
			type="text"
			bind:value={userInfo.username}
			placeholder="mrbills"
			autocomplete="username"
			maxlength="255"
			use:validators={[required]}
		/>
		<ThemedValidationHint
			hintProps={{ for: 'username', on: 'required' }}
			msg={REQUIRED_VIOLATION_MSG}
		/>
	</div>
	<div class="mb-4">
		<label class="label" for="email">Email</label>
		<input
			class="input"
			id="email"
			name="email"
			type="email"
			bind:value={userInfo.email}
			placeholder="payingbills@email.com"
			autocomplete="email"
			maxlength="320"
			use:validators={[required, email]}
		/>
		<HintGroup for="email">
			<ThemedValidationHint hintProps={{ on: 'required' }} msg={REQUIRED_VIOLATION_MSG} />
			<ThemedValidationHint
				hintProps={{ on: 'email', hideWhenRequired: true }}
				msg={EMAIL_VIOLATION_MSG}
			/>
		</HintGroup>
	</div>
	<div class="mb-4">
		<label class="label" for="password">Password</label>
		<input
			class="input"
			id="password"
			name="password"
			type="password"
			bind:value={userInfo.password}
			placeholder="**********"
			autocomplete="new-password"
			maxlength="120"
			use:validators={[required, pattern(STRONG_PASSWORD_PATTERN)]}
			on:keyup={validateConfirmPassword}
		/>
		<HintGroup for="password">
			<ThemedValidationHint hintProps={{ on: 'required' }} msg={REQUIRED_VIOLATION_MSG} />
			<ThemedValidationHint
				hintProps={{ on: 'pattern', hideWhenRequired: true }}
				msg="It is too easy to find out."
			/>
		</HintGroup>
	</div>
	<div class="mb-6">
		<label class="label" for="password">Confirm password</label>
		<input
			class="input"
			id="confirmPassword"
			name="confirmPassword"
			type="password"
			bind:value={userInfo.confirmPassword}
			placeholder="**********"
			autocomplete="new-password"
			maxlength="120"
			use:validators={[required, passwordMatch]}
		/>
		<HintGroup for="confirmPassword">
			<ThemedValidationHint hintProps={{ on: 'required' }} msg={REQUIRED_VIOLATION_MSG} />
			<ThemedValidationHint
				hintProps={{ on: 'passwordMatch', hideWhenRequired: true }}
				msg="It does not match the one above."
			/>
		</HintGroup>
	</div>
	{#if $formState === 'TECHNICAL_ERROR'}
		<AlertMsg msg={TECHNICAL_ERROR_ALERT_MSG} />
	{/if}
	<div class="mt-4 flex items-center justify-between">
		<button type="submit" class="btn variant-filled-primary">Register account</button>
		<button class="btn variant-filled-secondary" on:click|preventDefault={navigateToLogin}
			>Go to login</button
		>
	</div>
</form>
