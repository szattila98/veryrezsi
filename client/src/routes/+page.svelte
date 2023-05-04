<script lang="ts">
	import type { LayoutServerData } from './$types';

	export let data: LayoutServerData;
</script>

<div class="bg-slate-200 p-5">
	<h1 class="text-center text-4xl m-10 font-medium leading-tight">What would you like to do?</h1>
	<div class="flex flex-wrap text-center uppercase select-none">
		<div class="basis-1/3 flex-grow flex-1">
			<div class="my-6 mx-32 h-32 outline shadow-md">
				Start tracking a subscription
			</div>
		</div>
		<div class="basis-1/3 flex-grow flex-1">
			<div class="my-6 mx-32 h-32 outline shadow-md">
				Add a payment
			</div>
		</div>
		<div class="basis-1/3 flex-grow flex-1">
			<div class="my-6 mx-32 h-32 outline shadow-md">
				Check my dashboard
			</div>
		</div>
	</div>
</div>
