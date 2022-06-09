<script context="module" lang="ts">
	import type { Load } from '@sveltejs/kit';
	import { loadUser } from '$lib/auth';

	// Loads user prop in render time
	export const load: Load = loadUser;
</script>

<div class="content">
	<h1>Very Rezsi, So Profit!</h1>
</div>

<style lang="scss">
	.content {
		display: flex;
		flex-direction: column;
		align-items: center;
	}
</style>
