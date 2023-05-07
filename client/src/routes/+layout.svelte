<script lang="ts">
	import type { LayoutServerData } from './$types';
	import { get } from 'svelte/store';
	
	// -- Skeleton --
	// Your selected Skeleton theme:
	import '@skeletonlabs/skeleton/themes/theme-skeleton.css';
	// This contains the bulk of Skeletons required styles:
	import '@skeletonlabs/skeleton/styles/all.css';
	// Finally, your application's global stylesheet (sometimes labeled 'app.css')
	import '../app.css';

	import Footer from '$lib/components/layout-base/Footer.svelte';
	import Header from '$lib/components/layout-base/Header.svelte';
	import { loginSession } from '../stores';

	export let data: LayoutServerData;

	// If returning from different website, runs once (as it's an SPA) to restore user session if session cookie is still valid
	const { user } = data;
	$loginSession = user;

	export let session = get(loginSession);
</script>

<div class="flex min-h-screen flex-col">
	{#if session}
		<Header />
	{/if}

	<main class="flex-grow">
		<slot />
	</main>

	<Footer />
</div>
