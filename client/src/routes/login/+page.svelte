<script lang="ts">
	import { goto } from '$app/navigation';
	import { page } from '$app/stores';
	import type { LoginRequestData } from '$shared/api/login';

	const credentials: LoginRequestData = {
		email: '',
		password: ''
	};

	let message: string;

	async function login() {
		message = '';
		const form = <HTMLFormElement>document.getElementById('signIn');
		if (form.checkValidity()) {
			try {
				await callLoginApi(credentials);
			} catch (err) {
				if (err instanceof Error) {
					console.error('Login error', err.message);
					message = err.message;
				}
			}
		} else {
			form.classList.add('was-validated');
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
		goto('/register');
	}
</script>

<svelte:head>
	<title>Login to VeryRezsi</title>
	<meta name="robots" content="noindex, nofollow" />
</svelte:head>

<div>
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
			>
				<div class="mb-4">
					<label class="mb-2 block font-bold tracking-wide text-gray-700" for="email">Email</label>
					<input
						class="focus:shadow-outline w-full appearance-none rounded border py-2 px-3 leading-tight text-gray-700 shadow focus:outline-none"
						id="email"
						type="email"
						bind:value={credentials.email}
						placeholder="payingbills@email.com"
						autocomplete="email"
						required
					/>
				</div>
				<div class="mb-6">
					<label class="mb-2 block font-bold tracking-wide text-gray-700" for="password"
						>Password</label
					>
					<input
						class="focus:shadow-outline mb-3 w-full appearance-none rounded border py-2 px-3 leading-tight text-gray-700 shadow focus:outline-none"
						id="password"
						type="password"
						bind:value={credentials.password}
						placeholder="**********"
						required
					/>
				</div>
				<div class="flex items-center justify-between">
					{#if message}
						<p class="text-danger">{message}</p>
					{/if}
					<button
						class="focus:shadow-outline rounded bg-blue-500 py-2 px-4 text-white hover:bg-blue-700 focus:outline-none"
						on:click|preventDefault={login}>Sign In</button
					>
					<button
						class="focus:shadow-outline rounded bg-red-400 py-2 px-4 text-white hover:bg-red-600 focus:outline-none"
						on:click|preventDefault={navigateToRegister}>Go to registration</button
					>
				</div>
			</form>
			<p class="text-center text-xs text-gray-500">&copy;2023 VeryRezsi</p>
		</div>
	</div>
</div>

<style>
</style>
