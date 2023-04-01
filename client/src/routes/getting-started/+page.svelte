<script lang="ts">
	import { fade } from 'svelte/transition';
	import LoginForm from "$lib/components/getting-started/LoginForm.svelte";
	import RegistrationForm from "$lib/components/getting-started/RegistrationForm.svelte";

	let isRegister = false;

	const switchView = () => {
		isRegister = !isRegister
	}
</script>

<div>
	{#if isRegister}
		<div transition:fade="{{ duration: 200 }}">
			<RegistrationForm on:switch={switchView}/>
		</div>
	{:else}
		<div transition:fade="{{ duration: 200 }}">
			<LoginForm on:switch={switchView}/>
		</div>
	{/if}
</div>
