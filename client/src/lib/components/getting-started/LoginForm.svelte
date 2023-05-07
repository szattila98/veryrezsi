<script lang="ts">
	import { page } from '$app/stores';
	import type { LoginRequestData } from '$shared/api/login';
	import {
		EMAIL_VIOLATION_MSG,
		REQUIRED_VIOLATION_MSG,
		TECHNICAL_ERROR_ALERT_MSG,
		UNSUCCESFUL_LOGIN_ALERT_MSG
	} from '$shared/constants';
	import { createEventDispatcher } from 'svelte';
	import { useForm, Hint, validators, required, email, HintGroup } from 'svelte-use-form';
	import AlertMsg from '../common/AlertMsg.svelte';
	import ThemedValidationHint from '../common/ThemedValidationHint.svelte';
	import { createFormState, type BaseFormStates } from '$shared/composables/createFormState';

	const dispatch = createEventDispatcher<{ switchView: void }>();
	type FormStates = 'INVALID_CREDENTIALS' | BaseFormStates;
	const { formState, setFormState } = createFormState<FormStates>();

	const form = useForm({ email: {}, password: {} });

	const credentials: LoginRequestData = {
		email: '',
		password: ''
	};

	async function login() {
		$form.touched = true;
		if ($form.valid) {
			try {
				await callLoginApi(credentials);
			} catch (err) {
				console.error('Login error', err);
			}
		}
	}

	async function callLoginApi(credentials: LoginRequestData) {
		try {
			const res = await fetch('/api/user/login', {
				method: 'POST',
				body: JSON.stringify(credentials)
			});
			if (res.ok) {
				const referrer = $page.url.searchParams.get('referrer');
				if (referrer) return (window.location.href = referrer);
				window.location.href = '/';
			} else if (res.status === 401) {
				setFormState('INVALID_CREDENTIALS');
			} else {
				throw new Error('Invalid api response');
			}
		} catch (err) {
			setFormState('TECHNICAL_ERROR');
			if (err instanceof Error) {
				throw new Error('Sorry, you need to wait until we fix this', err);
			}
		}
	}

	function navigateToRegister() {
		dispatch('switchView');
	}
</script>

<svelte:head>
	<title>Login to VeryRezsi</title>
	<meta name="robots" content="noindex, nofollow" />
</svelte:head>

<form
	class="card variant-filled-surface p-8"
	id="signIn"
	autocomplete="on"
	novalidate
	use:form
	on:submit|preventDefault={login}
>
	<div class="mb-4">
		<label class="label" for="email">Email</label>
		<input
			class="input"
			id="email"
			name="email"
			type="email"
			bind:value={credentials.email}
			placeholder="payingbills@email.com"
			autocomplete="email"
			maxlength="320"
			use:validators={[required, email]}
		/>
		<HintGroup for="email">
			<ThemedValidationHint hintProps={{ on: "required" }} msg={REQUIRED_VIOLATION_MSG} />
			<ThemedValidationHint hintProps={{ on: "email", hideWhenRequired: true }} msg={EMAIL_VIOLATION_MSG} />
		</HintGroup>
	</div>
	<div class="mb-6">
		<label class="label" for="password">Password</label>
		<input
			class="input"
			id="password"
			name="password"
			type="password"
			autocomplete="current-password"
			bind:value={credentials.password}
			placeholder="**********"
			maxlength="120"
			use:validators={[required]}
		/>
		<ThemedValidationHint hintProps={{ for:"password", on: "required" }} msg={REQUIRED_VIOLATION_MSG} />
	</div>
	{#if  $formState === 'INVALID_CREDENTIALS'}
		<AlertMsg msg={UNSUCCESFUL_LOGIN_ALERT_MSG} />
	{/if}
	{#if $formState === 'TECHNICAL_ERROR'}
		<AlertMsg msg={TECHNICAL_ERROR_ALERT_MSG} />
	{/if}
	<div class="flex items-center justify-between mt-4">
		<button
			type="submit"
			class="btn variant-filled-primary"
			>Sign In
		</button>
		<button
			class="btn variant-filled-secondary"
			on:click|preventDefault={navigateToRegister}>Go to registration</button
		>
	</div>
</form>
