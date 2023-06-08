<script>
	import { AppBar } from '@skeletonlabs/skeleton';
	import { LightSwitch } from '@skeletonlabs/skeleton';

	import FaClipboardList from 'svelte-icons/fa/FaClipboardList.svelte';
	import FaRegUserCircle from 'svelte-icons/fa/FaRegUserCircle.svelte';
	import IoMdHome from 'svelte-icons/io/IoMdHome.svelte';
	import IoIosLogOut from 'svelte-icons/io/IoIosLogOut.svelte';
	import NavEntry from './NavEntry.svelte';

	async function logout() {
		try {
			const res = await fetch('/api/user/logout', {
				method: 'POST'
			});
			if (res.ok) {
				window.location.href = '/login';
				return;
			} else {
				console.error('Failed to logout, welcome to Hotel California');
			}
		} catch (err) {
			if (err instanceof Error) {
				console.error('API error while trying to log you out', err.message);
				throw new Error('Sorry, you need to wait until we fix this');
			}
		}
	}
</script>

<AppBar class="z-30" shadow="shadow-md">
	<svelte:fragment slot="lead">
		<NavEntry text="VeryRezsi" href="/" iconSize={6} class="mr-4 text-lg font-semibold">
			<IoMdHome />
		</NavEntry>
		<LightSwitch />
	</svelte:fragment>
	<svelte:fragment slot="trail">
		<nav>
			<ul class="flex space-x-4">
				<li>
					<NavEntry text="Profile" href="/profile" class="hover:text-white">
						<FaRegUserCircle />
					</NavEntry>
				</li>
				<li>
					<NavEntry text="Logout" callOnClick={logout} class="hover:text-white">
						<IoIosLogOut />
					</NavEntry>
				</li>
			</ul>
		</nav>
	</svelte:fragment>
</AppBar>
