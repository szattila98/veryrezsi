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
	import {
		MAX_LENGTH_VIOLATION_MSG,
		REQUIRED_VIOLATION_MSG,
		EMAIL_VIOLATION_MSG,
		VALIDATION_MSG
	} from '$shared/constants';

	const dispatch = createEventDispatcher<{ switchView: void }>();
	const STRONG_PASSWORD_PATTERN =
		/^(?=.*[a-z])(?=.*[A-Z])(?=.*\d)(?=.*[@$!%*?&])[A-Za-z\d@$!%*?&]{8,120}$/;

	const form = useForm();

	const userInfo: RegisterRequestData = {
		email: '',
		username: '',
		password: '',
		confirmPassword: ''
	};

	const passwordMatch: Validator = (value, form) => {
		if (!form.password) {
			return { passwordMatch: 'Password field not found in the form' };
		}

		return value === form.password.value ? null : { passwordMatch: 'Passwords are not matching' };
	};

	async function register() {
		const form = <HTMLFormElement>document.getElementById('register');
		if (form.checkValidity()) {
			try {
				await callRegisterApi(userInfo);
			} catch (err) {
				if (err instanceof Error) {
					console.log(err.message);
				}
			}
		} else {
			form.classList.add('was-validated');
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
				const fromApi = await res.json();
				const reason = fromApi.reason;
				throw new Error('Failed to register: ' + reason);
			}
		} catch (err) {
			if (err instanceof Error) {
				console.error('API error while trying to register user', err.message);
				throw new Error('Sorry, you need to wait until we fix this');
			}
		}
	}

	function navigateToLogin() {
		dispatch('switchView');
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
		<label class="mb-2 block font-bold text-fontblack" for="email">Username</label>
		<input
			class="focus:shadow-outline w-full appearance-none rounded-t border py-2 px-3 leading-tight text-fontblack shadow focus:outline-none"
			id="username"
			name="username"
			type="text"
			bind:value={userInfo.username}
			placeholder="mrbills"
			autocomplete="username"
			use:validators={[required, maxLength(255)]}
		/>
		<HintGroup for="username">
			<Hint on="required" class={VALIDATION_MSG}>{REQUIRED_VIOLATION_MSG}</Hint>
			<Hint on="maxLength" hideWhenRequired class={VALIDATION_MSG}
				>{{ MAX_LENGTH_VIOLATION_MSG }}</Hint
			>
		</HintGroup>
	</div>
	<div class="mb-4">
		<label class="mb-2 block font-bold text-fontblack" for="email">Email</label>
		<input
			class="focus:shadow-outline w-full appearance-none rounded-t border py-2 px-3 leading-tight text-fontblack shadow focus:outline-none"
			id="email"
			name="email"
			type="email"
			bind:value={userInfo.email}
			placeholder="payingbills@email.com"
			autocomplete="email"
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
			use:validators={[required, pattern(STRONG_PASSWORD_PATTERN)]}
		/>
		<HintGroup for="password">
			<Hint on="required" class={VALIDATION_MSG}>{REQUIRED_VIOLATION_MSG}</Hint>
			<Hint on="pattern" hideWhenRequired class={VALIDATION_MSG}>It is too easy to find out.</Hint>
		</HintGroup>
	</div>
	<div class="mb-6">
		<label class="mb-2 block font-bold text-fontblack" for="password">Confirm password</label>
		<input
			class="focus:shadow-outline w-full appearance-none rounded-t border py-2 px-3 leading-tight text-fontblack shadow focus:outline-none"
			id="passwordConfirm"
			name="passwordConfirm"
			type="password"
			bind:value={userInfo.confirmPassword}
			placeholder="**********"
			autocomplete="new-password"
			use:validators={[required, passwordMatch]}
		/>
		<HintGroup for="passwordConfirm">
			<Hint on="required" class={VALIDATION_MSG}>{REQUIRED_VIOLATION_MSG}</Hint>
			<Hint on="passwordMatch" hideWhenRequired class={VALIDATION_MSG}
				>It does not match the one above.</Hint
			>
		</HintGroup>
	</div>
	<div class="flex items-center justify-between">
		<button
			type="submit"
			class="focus:shadow-outline rounded bg-primary py-2 px-4 font-bold text-white hover:bg-primarydark focus:outline-none"
			>Register account</button
		>
		<button
			class="focus:shadow-outline rounded bg-secondary py-2 px-4 font-bold text-white hover:bg-secondarydark focus:outline-none"
			on:click|preventDefault={navigateToLogin}>Go to login</button
		>
	</div>
</form>
