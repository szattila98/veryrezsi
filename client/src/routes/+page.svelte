<script lang="ts">
	import type { LayoutServerData } from './$types';

	export let data: LayoutServerData;
</script>

<h1 class="text-3xl font-bold underline hover:bg-red-600">Hello world! {data?.user?.username}</h1>
