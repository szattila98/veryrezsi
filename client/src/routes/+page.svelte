<script lang="ts">
	import type { LayoutServerData } from './$types';

	export let data: LayoutServerData;
</script>

<h1 class="text-3xl">Hello rezsi!</h1>
<p>Welcome back <span class="bold">{data?.user?.username}</span></p>
