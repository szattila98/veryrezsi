<script>
	import FaHome from 'svelte-icons/fa/FaHome.svelte';
	import FaClipboardList from 'svelte-icons/fa/FaClipboardList.svelte';
	import FaRegUserCircle from 'svelte-icons/fa/FaRegUserCircle.svelte';
	import IoIosLogOut from 'svelte-icons/io/IoIosLogOut.svelte';
	import NavEntry from './NavEntry.svelte';
	import { GRADIENT } from '$lib/shared/constants';
</script>

<header class={`mb-4 py-4 px-6 text-gray-100 ${GRADIENT}`}>
	<div class="flex items-center justify-between">
		<NavEntry text="Veryrezsi" href="/" iconSize={8} class="text-xl font-semibold">
			<FaHome />
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
					<NavEntry text="Logout" href="/logout" class="hover:text-white">
						<IoIosLogOut />
					</NavEntry>
				</li>
			</ul>
		</nav>
	</div>
</header>
