<script>
	import { GRADIENT } from '$lib/shared/constants';
	import FaGithub from 'svelte-icons/fa/FaGithub.svelte';
</script>

<footer class={`mt-4 py-4 px-6 text-gray-100 ${GRADIENT}`}>
	<div class="container mx-auto flex items-center justify-center">
		<p>hello@veryrezsi.com</p>
		<div class="mx-4 h-6 border-l border-gray-100" />
		<a
			href="https://github.com/szattila98/veryrezsi"
			target="_blank"
			rel="noopener noreferrer"
			class="text-gray-100 hover:text-white"
		>
			<div class="mr-2 h-8 w-8">
				<FaGithub />
			</div>
		</a>
	</div>
</footer>
