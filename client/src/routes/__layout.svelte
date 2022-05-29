<script lang="ts">
	import type { TopAppBarComponentDev } from '@smui/top-app-bar';

	import {onMount} from 'svelte';

	import LayoutGrid, { Cell } from '@smui/layout-grid';
	import Button, { Label as ButtonLabel } from '@smui/button';
	import TopAppBar, { Row, Section } from '@smui/top-app-bar';
	import IconButton from '@smui/icon-button';

	import { goto } from '$app/navigation';
	import { session, page } from '$app/stores';

	import '../style/app.scss';

	let darkTheme: boolean;
	let topAppBar: TopAppBarComponentDev;

	$: modeLabel = `Switch to ${darkTheme ? 'light' : 'dark'} mode`;
	$: modeIcon = darkTheme ? 'light_mode' : 'dark_mode';

	onMount(() => {
    	darkTheme = window.matchMedia('(prefers-color-scheme: dark)').matches;
  	})

	const toggleMode = () => (darkTheme = !darkTheme);

	const signout = async () =>  {
		await fetch('/api/logout', { method: 'POST' });
		location.reload();
	}

	const toHome = () => {
		goto('/');
	}

	const toProfile = () => {
		goto('/profile');
	}

	const toDashboard = () => {
		goto('/dashboard');
	}

	const isLoggedIn = () => {
		return !!$session.user;
	};

	$: isCurrentPage = (pageName: string) => {
		return $page.url.pathname === pageName;
	}
</script>

<svelte:head>
	<title>VeryRezsi</title>

	{#if darkTheme === undefined}
		<link
			rel="stylesheet"
			href="/smui.css"
			media="(prefers-color-scheme: light)"
		/>
		<link
			rel="stylesheet"
			href="/smui-dark.css"
			media="screen and (prefers-color-scheme: dark)"
		/>
	{:else if darkTheme}
		<link rel="stylesheet" href="/smui.css" media="print" />
		<link rel="stylesheet" href="/smui-dark.css" media="screen" />
	{:else}
		<link rel="stylesheet" href="/smui.css" />
	{/if}
</svelte:head>

{#if isLoggedIn()}
	<TopAppBar id="appbar" bind:this={topAppBar} variant="standard">
		<Row>
			<Section align="start" toolbar>
				<Button on:click={toHome} disabled={isCurrentPage('/')} variant={isCurrentPage('/') ? 'outlined' : 'text'} >
					<ButtonLabel>Home</ButtonLabel>
				</Button>
				<Button on:click={toProfile} disabled={isCurrentPage('/profile') } variant={isCurrentPage('/profile') ? 'outlined' : 'text'}>
					<ButtonLabel>Profile</ButtonLabel>
				</Button>
				<Button on:click={toDashboard} disabled={isCurrentPage('/dashboard')} variant={isCurrentPage('/dashboard') ? 'outlined' : 'text'}>
					<ButtonLabel>Dashboard</ButtonLabel>
				</Button>
			</Section>
			<Section align="end" toolbar>
				<IconButton
				  aria-label="{modeLabel}"
				  class="material-icons"
				  on:click="{toggleMode}"
				  title="{modeLabel}"
				>
				  {modeIcon}
				</IconButton>
				<Button on:click={signout}>
					<ButtonLabel>Sign out</ButtonLabel>
				</Button>
			</Section>
		</Row>
  	</TopAppBar>
{/if}

<LayoutGrid>
	<Cell span={2} />
	<Cell span={8} align="middle">
		<slot />
	</Cell>
	<Cell span={2} />
</LayoutGrid>

<style lang="scss" global>
	app, body, html {
	  margin: 0
	}

	#appbar {
		position: static;
	}
  </style>