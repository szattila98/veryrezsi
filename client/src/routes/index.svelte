<script context="module" lang="ts">
	import type { Load } from '@sveltejs/kit';
	import { authLoad } from '$lib/auth';

	export const load: Load = authLoad;
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
