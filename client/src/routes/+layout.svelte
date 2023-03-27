<script lang="ts">
	import type { LayoutServerData } from './$types';

	import '../app.css';
	import { loginSession } from '../stores';

	export let data: LayoutServerData;

	// If returning from different website, runs once (as it's an SPA) to restore user session if session cookie is still valid
	const { user } = data;
	$loginSession = user;
</script>

<slot />
