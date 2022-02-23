<script>
	import LayoutGrid, { Cell } from '@smui/layout-grid';
	import '../style/app.scss';
</script>

<svelte:head>
	<title>VeryRezsi</title>
</svelte:head>

<a href="/register">Sign up!</a>
<a href="/login">Sign in!</a>

<LayoutGrid>
	<Cell span={2} />
	<Cell span={8} align="middle">
		<slot />
	</Cell>
	<Cell span={2} />
</LayoutGrid>
