<script lang="ts">
	import { goto } from '$app/navigation';
	import type { RegisterRequestData } from '$shared/api/register';

	const userInfo: RegisterRequestData = {
		email: '',
		username: '',
		password: '',
		confirmPassword: ''
	};

	let message: string;

	async function register() {
		message = '';
		const form = <HTMLFormElement>document.getElementById('register');
		if (form.checkValidity()) {
			try {
				await callRegisterApi(userInfo);
			} catch (err) {
				if (err instanceof Error) {
					message = err.message;
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
				return goto('/login');
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
		goto('/login');
	}
</script>

<svelte:head>
	<title>Register to VeryRezsi</title>
	<meta name="robots" content="noindex, nofollow" />
</svelte:head>

<div>
	<div class="flex h-screen items-center justify-center">
		<div class="w-full max-w-md">
			<div class="my-8 text-center text-4xl font-light">
				Register to <span class="text-blue-600">Very</span>Rezsi
			</div>
			<form
				class="mb-4 rounded bg-white px-8 pt-6 pb-8 shadow-md"
				id="register"
				autocomplete="on"
				novalidate
			>
				<div class="mb-4">
					<label class="mb-2 block font-bold text-gray-700" for="email">Username</label>
					<input
						class="focus:shadow-outline w-full appearance-none rounded border py-2 px-3 leading-tight text-gray-700 shadow focus:outline-none"
						id="username"
						type="text"
						bind:value={userInfo.username}
						placeholder="mrbills"
						required
						autocomplete="username"
					/>
				</div>
				<div class="mb-4">
					<label class="mb-2 block font-bold text-gray-700" for="email">Email</label>
					<input
						class="focus:shadow-outline w-full appearance-none rounded border py-2 px-3 leading-tight text-gray-700 shadow focus:outline-none"
						id="email"
						type="email"
						bind:value={userInfo.email}
						placeholder="payingbills@email.com"
						autocomplete="email"
						required
					/>
				</div>
				<div class="mb-4">
					<label class="mb-2 block font-bold text-gray-700" for="password">Password</label>
					<input
						class="focus:shadow-outline w-full appearance-none rounded border py-2 px-3 leading-tight text-gray-700 shadow focus:outline-none"
						id="password"
						type="password"
						bind:value={userInfo.password}
						placeholder="**********"
						autocomplete="new-password"
						required
					/>
				</div>
				<div class="mb-6">
					<label class="mb-2 block font-bold text-gray-700" for="password">Confirm password</label>
					<input
						class="focus:shadow-outline mb-3 w-full appearance-none rounded border py-2 px-3 leading-tight text-gray-700 shadow focus:outline-none"
						id="passwordConfirm"
						type="password"
						bind:value={userInfo.confirmPassword}
						placeholder="**********"
						autocomplete="new-password"
						required
					/>
				</div>
				<div class="flex items-center justify-between">
					{#if message}
						<p class="text-danger">{message}</p>
					{/if}
					<button
						class="focus:shadow-outline rounded bg-blue-500 py-2 px-4 font-bold text-white hover:bg-blue-700 focus:outline-none"
						on:click|preventDefault={register}>Register account</button
					>
					<button
						class="focus:shadow-outline rounded bg-green-500 py-2 px-4 font-bold text-white hover:bg-green-600 focus:outline-none"
						on:click|preventDefault={navigateToLogin}>Go to login</button
					>
				</div>
			</form>
			<p class="text-center text-xs text-gray-500">&copy;2023 VeryRezsi</p>
		</div>
	</div>
</div>

<style>
</style>
