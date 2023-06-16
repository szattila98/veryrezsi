<script lang="ts">
	// TODO:
	export let newUser: boolean = true;
</script>

<div id="title-background" class="h-28 bg-primary-500 md:h-64">
	<h1 class="pt-2 text-center text-4xl font-semibold text-surface-50 md:pt-20">
		What can we do for you?
	</h1>
</div>

<div class="flex select-none flex-wrap justify-center text-center text-lg font-semibold uppercase">
	<div class="max-w-2xl flex-1 flex-grow basis-1/3">
		<a
			href="/start-tracking"
			class="card card-hover variant-filled mx-32 flex h-32 items-center justify-center p-4"
		>
			Start tracking a subscription
		</a>
	</div>
	<div class="max-w-2xl flex-1 flex-grow basis-1/3">
		<a
			href="/add-payment"
			class="card card-hover variant-filled mx-32 mt-4 flex h-32 items-center justify-center p-4 lg:mt-12"
		>
			Add a payment
		</a>
	</div>
	<div class="max-w-2xl flex-1 flex-grow basis-1/3">
		<a
			href="/dashboard"
			class="card card-hover variant-filled mx-32 my-4 flex h-32 items-center justify-center p-4 lg:mt-24"
		>
			Check my dashboard
		</a>
	</div>
</div>

{#if newUser}
	<div class="handwriting hidden select-none flex-wrap justify-center uppercase xl:flex">
		<div class="flex flex-1 flex-grow basis-1/3 items-start justify-end">
			<img class="w-24" src="/vectors/right-drawn-arrow.svg" alt="arrow" />
			<p class="mt-20 pl-8">Start here, you can track anything recurring</p>
		</div>
		<div class="flex flex-1 flex-grow basis-1/4 flex-col items-center justify-start">
			<img class="w-16" src="/vectors/down-drawn-arrow.svg" alt="arrow" />
			<p class="pt-8">Someone helped you to pay a bill? Great, add it here</p>
		</div>
		<div class="flex flex-1 flex-grow basis-1/3 justify-center">
			<p class="mt-20 pr-8">Looking for some graphs? This way</p>
			<img class="w-24" src="/vectors/left-drawn-arrow.svg" alt="arrow" />
		</div>
	</div>
{/if}

<style>
	#title-background {
		clip-path: polygon(0 0%, 100% 0, 100% 100%, 0 67%);
	}

	.handwriting {
		font-family: 'Caveat';
		font-weight: bold;
	}
</style>
