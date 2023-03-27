<script lang="ts">
	import type { LayoutServerData } from './$types';

	import Footer from '$lib/components/layout-base/Footer.svelte';
	import Header from '$lib/components/layout-base/Header.svelte';
	import '../app.css';
	import { loginSession } from '../stores';

	export let data: LayoutServerData;

	// If returning from different website, runs once (as it's an SPA) to restore user session if session cookie is still valid
	const { user } = data;
	$loginSession = user;
</script>

<div class="flex min-h-screen flex-col">
	<Header />
	<main class="flex-grow">
		<slot />
	</main>
	<Footer />
</div>
