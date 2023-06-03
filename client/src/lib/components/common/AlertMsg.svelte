<script lang="ts">
	import { ALERT_MSG } from '$shared/constants';

	export let msg: string;
</script>

<div role="alert" class={ALERT_MSG}>
	{msg}
</div>
