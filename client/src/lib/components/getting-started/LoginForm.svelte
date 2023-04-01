<script lang="ts">
	import { page } from '$app/stores';
	import type { LoginRequestData } from '$shared/api/login';
	import { VALIDATION_MSG } from '$shared/constants';
	import { createEventDispatcher } from 'svelte';
	import { useForm, Hint, validators, required, email, HintGroup } from "svelte-use-form";

	const dispatch = createEventDispatcher<{switch: void}>();

  	const form = useForm();

	const credentials: LoginRequestData = {
		email: '',
		password: ''
	};

	async function login() {
		if ($form.valid) {
			try {
				await callLoginApi(credentials);
			} catch (err) {
				if (err instanceof Error) {
					console.error('Login error', err.message);
				}
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
				return;
			} else {
				const apiResponse = await res.json();
				throw new Error('Failed to login: ' + apiResponse.message);
			}
		} catch (err) {
			if (err instanceof Error) {
				console.error('Login error', err);
				throw new Error(err.message);
			}
		}
	}

	function navigateToRegister() {
		dispatch('switch');
	}
</script>

<svelte:head>
	<title>Login to VeryRezsi</title>
	<meta name="robots" content="noindex, nofollow" />
</svelte:head>

<div class="flex h-screen items-center justify-center">
    <div class="w-full max-w-sm">
        <div class="my-8 text-center text-4xl font-light">
            Login to <span class="text-blue-600">Very</span>Rezsi
        </div>
        <form
            class="mb-4 rounded bg-white px-8 pt-6 pb-8 shadow-md"
            id="signIn"
            autocomplete="on"
            novalidate
            use:form
            on:submit|preventDefault={login}
        >
            <div class="mb-4">
                <label class="mb-2 block font-bold tracking-wide text-gray-700" for="email">Email</label>
                <input
                    class="focus:shadow-outline w-full appearance-none rounded-t border py-2 px-3 leading-tight text-gray-700 shadow focus:outline-none"
                    id="email"
                    name="email"
                    type="email"
                    bind:value={credentials.email}
                    placeholder="payingbills@email.com"
                    autocomplete="email"
                    use:validators={[required, email]}
                />
                <HintGroup for="email">
                    <Hint on="required" class={VALIDATION_MSG}>This is a mandatory field</Hint>
                    <Hint on="email" hideWhenRequired class={VALIDATION_MSG}>Email is not valid</Hint>
                </HintGroup>
            </div>
            <div class="mb-6">
                <label class="mb-2 block font-bold tracking-wide text-gray-700" for="password"
                    >Password</label
                >
                <input
                    class="focus:shadow-outline w-full appearance-none rounded-t border py-2 px-3 leading-tight text-gray-700 shadow focus:outline-none"
                    id="password"
                    name="password"
                    type="password"
                    bind:value={credentials.password}
                    placeholder="**********"
                    use:validators={[required]}
                />
                <Hint for="password" on="required" class={VALIDATION_MSG}>This is a mandatory field</Hint>
            </div>
            <div class="flex items-center justify-between">
                <button
                    type="submit"
                    class="focus:shadow-outline rounded bg-blue-500 py-2 px-4 text-white hover:bg-blue-700 focus:outline-none disabled:bg-gray-500"
                    >Sign In
                </button>
                <button
                    class="focus:shadow-outline rounded bg-green-500 py-2 px-4 text-white hover:bg-green-600 focus:outline-none"
                    on:click|preventDefault={navigateToRegister}>Go to registration</button
                >
            </div>
        </form>
    </div>
</div>