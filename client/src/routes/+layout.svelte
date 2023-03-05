<script>
	import Footer from '$lib/components/layout-base/Footer.svelte';
	import Header from '$lib/components/layout-base/Header.svelte';
	import '../app.css';
</script>

<div class="flex min-h-screen flex-col">
	<Header />
	<main class="flex-grow">
		<slot />
	</main>
	<Footer />
</div>
