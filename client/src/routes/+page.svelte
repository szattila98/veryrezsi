<script lang="ts">
	import type { LayoutServerData } from './$types';

	export let data: LayoutServerData;

	// TODO: 
	export let newUser: boolean = true
</script>

<style>
	#title-background {
		clip-path: polygon(0 0%, 100% 0, 100% 100%, 0 67%);
	}

	.handwriting {
		font-family: 'Caveat';
		font-weight: bold;
	}
</style>

<div id="title-background" class="bg-primary-500 h-28 md:h-64">
	<h1 class="pt-2 md:pt-20 text-center text-4xl text-surface-50 font-semibold">What can we do for you?</h1>
</div>
	
<div class="flex flex-wrap justify-center text-center uppercase select-none text-lg font-semibold">
	<div class="basis-1/3 flex-grow flex-1 max-w-2xl">
		<a href="/start-tracking" class="card card-hover variant-filled h-32 mx-32 p-4 flex justify-center items-center">
			Start tracking a subscription
		</a>
	</div>
	<div class="basis-1/3 flex-grow flex-1 max-w-2xl">
		<a href="/add-payment" class="card card-hover variant-filled h-32 mt-4 lg:mt-12 mx-32 p-4 flex justify-center items-center">
			Add a payment
		</a>
	</div>
	<div class="basis-1/3 flex-grow flex-1 max-w-2xl">
		<a href="/dashboard" class="card card-hover variant-filled h-32 my-4 lg:mt-24 mx-32 p-4 flex justify-center items-center">
			Check my dashboard
		</a>
	</div>
</div>

{#if newUser}
	<div class="hidden xl:flex flex-wrap justify-center uppercase select-none handwriting">
		<div class="basis-1/3 flex-grow flex-1 flex justify-end items-start">
			<img class="w-24" src="/vectors/right-drawn-arrow.svg" alt="arrow"/>
			<p class="pl-8 mt-20">Start here, you can track anything recurring</p>
		</div>
		<div class="basis-1/4 flex-grow flex-1 flex flex-col justify-start items-center">
			<img class="w-16" src="/vectors/down-drawn-arrow.svg" alt="arrow"/>
			<p class="pt-8">Someone helped you to pay a bill? Great, add it here</p>
		</div>
		<div class="basis-1/3 flex-grow flex-1 flex justify-center ">
			<p class="pr-8 mt-20">Looking for some graphs? This way</p>
			<img class="w-24" src="/vectors/left-drawn-arrow.svg" alt="arrow"/>
		</div>
	</div>
{/if}
