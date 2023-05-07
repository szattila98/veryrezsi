<script lang="ts" context="module">
	import { page } from '$app/stores';

	import FaRegSadTear from 'svelte-icons/fa/FaRegSadTear.svelte'
	import FaSearchengin from 'svelte-icons/fa/FaSearchengin.svelte'
</script>

<div class="flex justify-center items-center mt-8 h-[200px]">
	<div class="alert variant-ringed p-8">
		{#if $page.status == 404}
		<div class="w-fit h-16">
			<FaSearchengin />
		</div>
		<div class="alert-message">
			<h3>Page not found</h3>
			Looks like we have pointed you to a wrong direction.
		</div>
		{:else}
		<div class="w-fit h-16">
			<FaRegSadTear />
		</div>
		<div class="alert-message">
			<h3>{$page.error?.message}</h3>
			Something went horribly wrong. Next round on us!
		</div>
		{/if}
		
	</div>
</div>


