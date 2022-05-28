<script context="module" lang="ts">
	import type { Load } from '@sveltejs/kit';
	import type { user } from "$shared/domain";

	import { authLoad } from "$lib/auth";

	export const load: Load = authLoad;
</script>

<script lang="ts">
	export let user: user;

	// If you want to get user without the ugly context="module" block
	// import { session } from '$app/stores';
	// export let user = $session.user;
</script>

<div class="content">
	<h1>Very Rezsi, So Profit!</h1>
	<p>Your email: {user.email}</p>
</div>

<style lang="scss">
	.content {
		display: flex;
		flex-direction: column;
		align-items: center;
	}
</style>
