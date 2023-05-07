<script lang="ts">
	import type { LayoutServerData } from './$types';

	export let data: LayoutServerData;
</script>

<style>
	#title-background {
		clip-path: polygon(0 0%, 100% 0, 100% 100%, 0 67%);
	}
</style>

<div id="title-background" class="bg-primary-500 h-28 md:h-64">
	<h1 class="pt-2 md:pt-20 text-center text-4xl text-surface-50 font-semibold">What can we do for you?</h1>
</div>
	
<div class="flex flex-wrap justify-center text-center uppercase select-none text-lg font-semibold">
	<div class="basis-1/3 flex-grow flex-1 max-w-2xl">
		<a href="/start-tracking" class="card card-hover variant-ringed-secondary h-32 mx-32 p-4 flex justify-center items-center">
			Start tracking a subscription
		</a>
	</div>
	<div class="basis-1/3 flex-grow flex-1 max-w-2xl">
		<a href="/add-payment" class="card card-hover variant-ringed-secondary h-32 mt-4 lg:mt-12 mx-32 p-4 flex justify-center items-center">
			Add a payment
		</a>
	</div>
	<div class="basis-1/3 flex-grow flex-1 max-w-2xl">
		<a href="/dashboard" class="card card-hover variant-ringed-secondary h-32 my-4 lg:mt-24 mx-32 p-4 flex justify-center items-center">
			Check my dashboard
		</a>
	</div>
</div>
