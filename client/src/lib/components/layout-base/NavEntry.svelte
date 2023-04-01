<script lang="ts">
	export let text: string;
	export let href: string | null = null;
	export let callOnClick: (() => void) | null = null;
	export let iconSize = 6;

	export function callIfDefined(toCall: (() => void) | null) {
		if (toCall) {
			toCall();
		}
	}
</script>

{#if callOnClick}
	<!-- svelte-ignore a11y-click-events-have-key-events -->
	<button
		on:click={() => {
			callIfDefined(callOnClick);
		}}
		class="flex flex-row align-middle"
	>
		<div class={`mr-2 h-${iconSize} w-${iconSize}`}>
			<slot />
		</div>
		<span class={`${$$props.class}`}>{text}</span>
	</button>
{:else if href}
	<a {href} class="flex flex-row align-middle">
		<div class={`mr-2 h-${iconSize} w-${iconSize}`}>
			<slot />
		</div>
		<span class={`${$$props.class}`}>{text}</span>
	</a>
{/if}
