<script lang="ts">
	import { page } from '$app/stores';
	import { scale } from 'svelte/transition';
	import { cubicInOut } from 'svelte/easing';
	import LoginForm from '$lib/components/getting-started/LoginForm.svelte';
	import RegistrationForm from '$lib/components/getting-started/RegistrationForm.svelte';

	const url = $page.url;
	const withRegisterURL = url.search.includes('register');
	$: isRegister = withRegisterURL;

	const switchView = () => {
		isRegister = !isRegister;
	};
</script>

<div class="py-12">
	<div class="my-10 text-center text-4xl font-light">
		Start <span class="font-semibold">VeryRezsi</span>
	</div>
	{#if isRegister}
		<div class="m-auto max-w-sm" in:scale={{ duration: 200, easing: cubicInOut }}>
			<RegistrationForm on:switchView={switchView} />
		</div>
	{:else}
		<div class="m-auto max-w-sm" in:scale={{ duration: 200, easing: cubicInOut }}>
			<LoginForm on:switchView={switchView} />
		</div>
	{/if}
</div>
