<script lang="ts">
	import { createEventDispatcher } from 'svelte';
	import type { RegisterRequestData } from '$shared/api/register';
	import type { Validator } from 'svelte-use-form';
	import {
		useForm,
		Hint,
		validators,
		required,
		maxLength,
		email,
		HintGroup,
		pattern
	} from 'svelte-use-form';
	import { REQUIRED_VIOLATION_MSG, EMAIL_VIOLATION_MSG, VALIDATION_MSG } from '$shared/constants';

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

	const passwordMatch: Validator = (value, form) => {
		return value === form.password?.value ? null : { passwordMatch: 'Passwords are not matching' };
	};

	async function register() {
		$form.touched = true;
		if ($form.valid) {
			try {
				await callRegisterApi(userInfo);
			} catch (err) {
				console.log('Registration error', err);
			}
		}
	}

	async function callRegisterApi(userInfo: RegisterRequestData) {
		try {
			const res = await fetch('/api/register', {
				method: 'POST',
				body: JSON.stringify(userInfo)
			});
			if (res.ok) {
				return navigateToLogin();
			} else {
				const apiResponse = await res.json();
				throw new Error('Failed to register: ' + apiResponse.message);
			}
		} catch (err) {
			console.error('Registration', err);
			throw new Error('Sorry, you need to wait until we fix this');
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
	class="rounded bg-white px-8 pt-6 pb-8 shadow-md"
	id="register"
	autocomplete="on"
	novalidate
	use:form
	on:submit|preventDefault={register}
>
	<div class="mb-4">
		<label class="text-fontblack mb-2 block font-bold" for="email">Username</label>
		<input
			class="focus:shadow-outline text-fontblack w-full appearance-none rounded-t border py-2 px-3 leading-tight shadow focus:outline-none"
			id="username"
			name="username"
			type="text"
			bind:value={userInfo.username}
			placeholder="mrbills"
			autocomplete="username"
			maxlength="255"
			use:validators={[required]}
		/>
		<Hint on="required" class={VALIDATION_MSG}>{REQUIRED_VIOLATION_MSG}</Hint>
	</div>
	<div class="mb-4">
		<label class="text-fontblack mb-2 block font-bold" for="email">Email</label>
		<input
			class="focus:shadow-outline text-fontblack w-full appearance-none rounded-t border py-2 px-3 leading-tight shadow focus:outline-none"
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
			<Hint on="required" class={VALIDATION_MSG}>{REQUIRED_VIOLATION_MSG}</Hint>
			<Hint on="email" hideWhenRequired class={VALIDATION_MSG}>{EMAIL_VIOLATION_MSG}</Hint>
		</HintGroup>
	</div>
	<div class="mb-4">
		<label class={`{BLACK_TEXT_COLOR} mb-2 block font-bold`} for="password">Password</label>
		<input
			class="focus:shadow-outline text-fontbalck w-full appearance-none rounded-t border py-2 px-3 leading-tight shadow focus:outline-none"
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
			<Hint on="required" class={VALIDATION_MSG}>{REQUIRED_VIOLATION_MSG}</Hint>
			<Hint on="pattern" hideWhenRequired class={VALIDATION_MSG}>It is too easy to find out.</Hint>
		</HintGroup>
	</div>
	<div class="mb-6">
		<label class="text-fontblack mb-2 block font-bold" for="password">Confirm password</label>
		<input
			class="focus:shadow-outline text-fontblack w-full appearance-none rounded-t border py-2 px-3 leading-tight shadow focus:outline-none"
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
			<Hint on="required" class={VALIDATION_MSG}>{REQUIRED_VIOLATION_MSG}</Hint>
			<Hint on="passwordMatch" hideWhenRequired class={VALIDATION_MSG}
				>It does not match the one above.</Hint
			>
		</HintGroup>
	</div>
	<div class="flex items-center justify-between">
		<button
			type="submit"
			class="focus:shadow-outline bg-primary hover:bg-primarydark rounded py-2 px-4 font-bold text-white focus:outline-none"
			>Register account</button
		>
		<button
			class="focus:shadow-outline bg-secondary hover:bg-secondarydark rounded py-2 px-4 font-bold text-white focus:outline-none"
			on:click|preventDefault={navigateToLogin}>Go to login</button
		>
	</div>
</form>
