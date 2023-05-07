<script>
	import FaGithub from 'svelte-icons/fa/FaGithub.svelte';
</script>

<footer class={`px-6 py-4 bg-surface-100-800-token z-30`}>
	<div class="container mx-auto flex items-center justify-center">
		<p class="font-light">hello@veryrezsi.com</p>
		<div class="mx-4 h-6 border-l border-gray-100" />
		<a
			href="https://github.com/szattila98/veryrezsi"
			target="_blank"
			rel="noopener noreferrer"
			class="hover:text-white"
		>
			<div class="mr-2 h-8 w-8 text-black hover:text-primary-50">
				<FaGithub />
			</div>
		</a>
	</div>
</footer>
