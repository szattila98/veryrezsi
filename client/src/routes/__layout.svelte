<script>
	import LayoutGrid, { Cell } from '@smui/layout-grid';
	import Button, { Label as ButtonLabel } from '@smui/button';
	import { session } from '$app/stores';

	import '../style/app.scss';

	async function signout() {
		await fetch('/api/logout', { method: 'POST' });

		location.reload();
	}

	const isLoggedIn = () => {
		return !!$session.user;
	};
</script>

<svelte:head>
	<title>VeryRezsi</title>
</svelte:head>

{#if isLoggedIn()}
	<Button on:click={signout} variant="raised">
		<ButtonLabel>Sign out</ButtonLabel>
	</Button>
{/if}

<LayoutGrid>
	<Cell span={2} />
	<Cell span={8} align="middle">
		<slot />
	</Cell>
	<Cell span={2} />
</LayoutGrid>
