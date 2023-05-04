<script>
	import FaClipboardList from 'svelte-icons/fa/FaClipboardList.svelte';
	import FaRegUserCircle from 'svelte-icons/fa/FaRegUserCircle.svelte';
	import IoMdHome from 'svelte-icons/io/IoMdHome.svelte';
	import IoIosLogOut from 'svelte-icons/io/IoIosLogOut.svelte';
	import NavEntry from './NavEntry.svelte';
	import { GRADIENT } from '$lib/shared/constants';

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

<header class={`px-6 py-4 text-gray-100 ${GRADIENT} shadow-xl z-30`}>
	<div class="flex flex-col items-center justify-between sm:flex-row">
		<NavEntry text="VeryRezsi" href="/" iconSize={8} class="text-xl font-semibold">
			<IoMdHome />
		</NavEntry>
		<nav>
			<ul class="flex space-x-4">
				<li>
					<NavEntry text="Dashboard" href="/dashboard" class="hover:text-white">
						<FaClipboardList />
					</NavEntry>
				</li>
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
	</div>
</header>
