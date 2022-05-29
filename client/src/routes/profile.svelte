<script context="module" lang="ts">
	import type { Load } from '@sveltejs/kit';
	import { authLoad } from '$lib/auth';

	export const load: Load = authLoad;
</script>

<script lang="ts">
	import type { user } from '$shared/domain';
	import UserDetails from '$lib/components/UserDetails.svelte';

	export let user: user;
</script>

<div class="content">
	<h1 id="title">Your profile</h1>
	<div id="details">
		<UserDetails {user} />
	</div>
</div>

<style lang="scss">
	.content {
		display: flex;
		flex-direction: column;
		align-items: center;
	}
</style>
