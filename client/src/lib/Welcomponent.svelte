<script lang="ts">
	import Button, { Label } from '@smui/button';

	let clicked = 0;
</script>

<h1 class="blue">Welcome to SvelteKit</h1>
<p class="red">
	Visit <a href="https://kit.svelte.dev">kit.svelte.dev</a> to read the documentation
</p>

<Button on:click={() => clicked++}>
	<Label>Clickmeeee</Label>
</Button>
<pre class="status">Clicked: {clicked}</pre>

<style lang="scss">
	@import '../style/app.scss';

	.red {
		color: red;
	}
</style>
