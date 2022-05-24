<script context="module" lang="ts">
	import type { Load } from '@sveltejs/kit';
	import { authLoad } from "$lib/auth";

	export const load: Load = authLoad;
</script>

<script lang="ts">
	import UserDetails from '$lib/components/UserDetails.svelte';

	export let user;

</script>

<div class="content">
	<h1>Your personal profile page</h1>


	<UserDetails user={user} />
</div>

<style lang="scss">
	.content {
		display: flex;
		flex-direction: column;
		align-items: center;
	}
</style>
