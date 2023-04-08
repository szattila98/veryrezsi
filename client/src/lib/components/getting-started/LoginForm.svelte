<script lang="ts">
	import { page } from '$app/stores';
	import type { LoginRequestData } from '$shared/api/login';
	import { EMAIL_VIOLATION_MSG, REQUIRED_VIOLATION_MSG, VALIDATION_MSG } from '$shared/constants';
	import { createEventDispatcher } from 'svelte';
	import { useForm, Hint, validators, required, email, HintGroup } from 'svelte-use-form';

	const dispatch = createEventDispatcher<{ switchView: void }>();

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
			const res = await fetch('/api/login', {
				method: 'POST',
				body: JSON.stringify(credentials)
			});
			if (res.ok) {
				const referrer = $page.url.searchParams.get('referrer');
				if (referrer) return (window.location.href = referrer);
				window.location.href = '/';
			} else {
				const apiResponse = await res.json();
				throw new Error('Failed to login: ' + apiResponse.message);
			}
		} catch (err) {
			console.error('Login error', err);
			throw new Error('Sorry, you need to wait until we fix this');
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
	class="rounded bg-white px-8 pt-6 pb-8 shadow-md"
	id="signIn"
	autocomplete="on"
	novalidate
	use:form
	on:submit|preventDefault={login}
>
	<div class="mb-4">
		<label class="text-fontblack mb-2 block font-bold tracking-wide" for="email">Email</label>
		<input
			class="focus:shadow-outline text-fontblack w-full appearance-none rounded-t border py-2 px-3 leading-tight shadow focus:outline-none"
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
		<label class="text-fontblack mb-2 block font-bold tracking-wide" for="password">Password</label>
		<input
			class="focus:shadow-outline text-fontblack w-full appearance-none rounded-t border py-2 px-3 leading-tight shadow focus:outline-none"
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
	<div class="flex items-center justify-between">
		<button
			type="submit"
			class="focus:shadow-outline bg-primary hover:bg-primarydark rounded py-2 px-4 text-white focus:outline-none disabled:bg-gray-500"
			>Sign In
		</button>
		<button
			class="focus:shadow-outline bg-secondary hover:bg-secondarydark rounded py-2 px-4 text-white focus:outline-none"
			on:click|preventDefault={navigateToRegister}>Go to registration</button
		>
	</div>
</form>
