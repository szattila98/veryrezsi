<script lang="ts">
	export let text: string;
	export let href: string;
	export let iconSize = 6;
</script>

<a {href} class="flex flex-row align-middle">
	<div class={`mr-2 h-${iconSize} w-${iconSize}`}>
		<slot />
	</div>
	<span class={`${$$props.class}`}>{text}</span>
</a>
