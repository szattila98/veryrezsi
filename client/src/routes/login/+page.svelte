<script lang="ts">
	import { goto } from '$app/navigation';
	import { page } from '$app/stores';
	import { loginSession } from '../../stores';
	import type { LoginRequestData } from '$shared/api/login';
	let message: string;
	const credentials: LoginRequestData = {
		email: '',
		password: ''
	};
	async function login() {
		message = '';
		const form = <HTMLFormElement>document.getElementById('signIn');
		if (form.checkValidity()) {
			try {
				await loginLocal(credentials);
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

	async function loginLocal(credentials: LoginRequestData) {
		try {
			const res = await fetch('/api/login', {
				method: 'POST',
				body: JSON.stringify(credentials),
				headers: {
					'Content-Type': 'application/json'
				}
			});
			const fromAPI = await res.json();
			if (res.ok) {
				loginSession.set(fromAPI.user);
				const referrer = $page.url.searchParams.get('referrer');
				if (referrer) return goto(referrer);
				return goto('/');
			} else {
				throw new Error(fromAPI.message);
			}
		} catch (err) {
			if (err instanceof Error) {
				console.error('Login error', err);
				throw new Error(err.message);
			}
		}
	}
</script>

<svelte:head>
	<title>Login to VeryRezsi</title>
	<meta name="robots" content="noindex, nofollow" />
</svelte:head>

<div>
	<div class="flex h-screen items-center justify-center">
		<div class="w-full max-w-xs">
			<form
				class="mb-4 rounded bg-white px-8 pt-6 pb-8 shadow-md"
				id="signIn"
				autocomplete="on"
				novalidate
			>
				<div class="mb-4">
					<label class="mb-2 block font-bold text-gray-700" for="email">Email</label>
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
					<label class="mb-2 block font-bold text-gray-700" for="password">Password</label>
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
						class="focus:shadow-outline rounded bg-blue-500 py-2 px-4 font-bold text-white hover:bg-blue-700 focus:outline-none"
						on:click|preventDefault={login}>Sign In</button
					>
				</div>
			</form>
			<p class="text-center text-xs text-gray-500">&copy;2023 VeryRezsi</p>
		</div>
	</div>
</div>

<style>
</style>
