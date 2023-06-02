<script lang="ts">
	import { page } from '$app/stores';
	import type { LoginRequestData } from '$shared/api/login';
	import {
		EMAIL_VIOLATION_MSG,
		REQUIRED_VIOLATION_MSG,
		TECHNICAL_ERROR_ALERT_MSG,
		UNSUCCESFUL_LOGIN_ALERT_MSG,
		VALIDATION_MSG
	} from '$shared/constants';
	import { createEventDispatcher } from 'svelte';
	import { useForm, Hint, validators, required, email, HintGroup } from 'svelte-use-form';
	import AlertMsg from '../common/AlertMsg.svelte';
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
	class="rounded bg-white px-8 pb-8 pt-6 shadow-md"
	id="signIn"
	autocomplete="on"
	novalidate
	use:form
	on:submit|preventDefault={login}
>
	<div class="mb-4">
		<label class="mb-2 block font-bold tracking-wide text-fontblack" for="email">Email</label>
		<input
			class="focus:shadow-outline w-full appearance-none rounded-t border px-3 py-2 leading-tight text-fontblack shadow focus:outline-none"
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
			<Hint on="required" class={VALIDATION_MSG}>{REQUIRED_VIOLATION_MSG}</Hint>
			<Hint on="email" hideWhenRequired class={VALIDATION_MSG}>{EMAIL_VIOLATION_MSG}</Hint>
		</HintGroup>
	</div>
	<div class="mb-6">
		<label class="mb-2 block font-bold tracking-wide text-fontblack" for="password">Password</label>
		<input
			class="focus:shadow-outline w-full appearance-none rounded-t border px-3 py-2 leading-tight text-fontblack shadow focus:outline-none"
			id="password"
			name="password"
			type="password"
			autocomplete="current-password"
			bind:value={credentials.password}
			placeholder="**********"
			maxlength="120"
			use:validators={[required]}
		/>
		<Hint for="password" on="required" class={VALIDATION_MSG}>This is a mandatory field</Hint>
	</div>
	{#if $formState === 'INVALID_CREDENTIALS'}
		<AlertMsg msg={UNSUCCESFUL_LOGIN_ALERT_MSG} />
	{/if}
	{#if $formState === 'TECHNICAL_ERROR'}
		<AlertMsg msg={TECHNICAL_ERROR_ALERT_MSG} />
	{/if}
	<div class="flex items-center justify-between">
		<button
			type="submit"
			class="focus:shadow-outline rounded bg-primary px-4 py-2 text-white hover:bg-primarydark focus:outline-none disabled:bg-gray-500"
			>Sign In
		</button>
		<button
			class="focus:shadow-outline rounded bg-secondary px-4 py-2 text-white hover:bg-secondarydark focus:outline-none"
			on:click|preventDefault={navigateToRegister}>Go to registration</button
		>
	</div>
</form>
