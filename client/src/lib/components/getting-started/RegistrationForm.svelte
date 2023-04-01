<script lang="ts">
	import { createEventDispatcher } from 'svelte';
	import type { RegisterRequestData } from '$shared/api/register';
	import type { ValidationErrors } from "svelte-use-form";
	import { useForm, Hint, validators, required, maxLength, email, HintGroup, pattern } from "svelte-use-form";
	import { VALIDATION_MSG } from '$shared/constants';

	const dispatch = createEventDispatcher<{switch: void}>();
	const STRONG_PASSWORD_PATTERN = /^(?=.*[a-z])(?=.*[A-Z])(?=.*\d)(?=.*[@$!%*?&])[A-Za-z\d@$!%*?&]{8,120}$/;

	const form = useForm();

	const userInfo: RegisterRequestData = {
		email: '',
		username: '',
		password: '',
		confirmPassword: ''
	};

	const passwordMatch = (value: string, form: any): null | ValidationErrors => {
        return value === form.password.value
            ? null
            : { passwordMatch: "Passwords are not matching" };
	}

	async function register() {
		const form = <HTMLFormElement>document.getElementById('register');
		if (form.checkValidity()) {
			try {
				await callRegisterApi(userInfo);
			} catch (err) {
				if (err instanceof Error) {
					console.log(err.message)
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
		dispatch('switch');
	}
</script>

<div class="flex h-screen items-center justify-center">
	<div class="w-full max-w-md">
		<div class="my-8 text-center text-4xl font-light">
			Register to <span class="text-blue-600">Very</span>Rezsi
		</div>
		<form
			class="mb-4 rounded bg-white px-8 pt-6 pb-8 shadow-md"
			id="register"
			autocomplete="on"
			use:form
			on:submit|preventDefault={register}
		>
			<div class="mb-4">
				<label class="mb-2 block font-bold text-gray-700" for="email">Username</label>
				<input
					class="focus:shadow-outline w-full appearance-none rounded-t border py-2 px-3 leading-tight text-gray-700 shadow focus:outline-none"
					id="username"
					name="username"
					type="text"
					bind:value={userInfo.username}
					placeholder="mrbills"
					autocomplete="username"
                    use:validators={[required, maxLength(255)]}
				/>
                <HintGroup for="username">
                    <Hint on="required" class={VALIDATION_MSG}>This is a mandatory field</Hint>
                    <Hint on="maxLength" hideWhenRequired class={VALIDATION_MSG}>Username is too long</Hint>
                </HintGroup>
			</div>
			<div class="mb-4">
				<label class="mb-2 block font-bold text-gray-700" for="email">Email</label>
				<input
					class="focus:shadow-outline w-full appearance-none rounded-t border py-2 px-3 leading-tight text-gray-700 shadow focus:outline-none"
					id="email"
					name="email"
					type="email"
					bind:value={userInfo.email}
					placeholder="payingbills@email.com"
					autocomplete="email"
					use:validators={[required, email]}
				/>
                <HintGroup for="email">
                    <Hint on="required" class={VALIDATION_MSG}>This is a mandatory field</Hint>
                    <Hint on="email" hideWhenRequired class={VALIDATION_MSG}>Email is not valid</Hint>
                </HintGroup>
			</div>
			<div class="mb-4">
				<label class="mb-2 block font-bold text-gray-700" for="password">Password</label>
				<input
					class="focus:shadow-outline w-full appearance-none rounded-t border py-2 px-3 leading-tight text-gray-700 shadow focus:outline-none"
					id="password"
					name="password"
					type="password"
					bind:value={userInfo.password}
					placeholder="**********"
					autocomplete="new-password"
					use:validators={[required, pattern(STRONG_PASSWORD_PATTERN)]}
				/>
                <HintGroup for="password">
                    <Hint on="required" class={VALIDATION_MSG}>This is a mandatory field</Hint>
                    <Hint on="pattern" hideWhenRequired class={VALIDATION_MSG}>Password is weak</Hint>
                </HintGroup>
			</div>
			<div class="mb-6">
				<label class="mb-2 block font-bold text-gray-700" for="password">Confirm password</label>
				<input
					class="focus:shadow-outline w-full appearance-none rounded-t border py-2 px-3 leading-tight text-gray-700 shadow focus:outline-none"
					id="passwordConfirm"
					name="passwordConfirm"
					type="password"
					bind:value={userInfo.confirmPassword}
					placeholder="**********"
					autocomplete="new-password"
					use:validators={[required, passwordMatch]}
				/>
                <HintGroup for="passwordConfirm">
                    <Hint on="required" class={VALIDATION_MSG}>This is a mandatory field</Hint>
                    <Hint on="passwordMatch" hideWhenRequired class={VALIDATION_MSG}>Passwords do not match</Hint>
                </HintGroup>
			</div>
			<div class="flex items-center justify-between">
				<button
                    type="submit"
					class="focus:shadow-outline rounded bg-blue-500 py-2 px-4 font-bold text-white hover:bg-blue-700 focus:outline-none"
                >Register account</button
				>
				<button
					class="focus:shadow-outline rounded bg-green-500 py-2 px-4 font-bold text-white hover:bg-green-600 focus:outline-none"
					on:click|preventDefault={navigateToLogin}>Go to login</button
				>
			</div>
		</form>
	</div>
</div>
