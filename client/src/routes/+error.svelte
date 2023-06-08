<script lang="ts" context="module">
	import { page } from '$app/stores';

	import FaRegSadTear from 'svelte-icons/fa/FaRegSadTear.svelte';
	import FaSearchengin from 'svelte-icons/fa/FaSearchengin.svelte';
</script>

<div class="mt-8 flex h-[200px] items-center justify-center">
	<div class="alert variant-ringed p-8">
		{#if $page.status == 404}
			<div class="h-16 w-fit">
				<FaSearchengin />
			</div>
			<div class="alert-message">
				<h3>Page not found</h3>
				Looks like we have pointed you to a wrong direction.
			</div>
		{:else}
			<div class="h-16 w-fit">
				<FaRegSadTear />
			</div>
			<div class="alert-message">
				<h3>{$page.error?.message}</h3>
				Something went horribly wrong. Next round on us!
			</div>
		{/if}
	</div>
</div>
