<script lang="ts">
	import { Hint } from 'svelte-use-form';
	import IoIosArrowUp from 'svelte-icons/io/IoIosArrowUp.svelte';

	export let msg: string;
	export let hintProps: object = {};
</script>

<Hint {...hintProps} class="alert variant-ghost-warning rounded-tl-none rounded-tr-none p-2">
	<div class="h-6">
		<IoIosArrowUp />
	</div>
	<div class="alert-message">
		{msg}
	</div>
</Hint>
