<script lang="ts">
	import IconButton from '@smui/icon-button';

	export let user;

</script>

<div id="userDetails">
	<div>
		<p>Username</p>
		<p>{user.username}</p>
	</div>
	<div>
		<p>Email</p>
		<p>{user.email}</p>
	</div>

</div>


<style lang="scss">
#userDetails {
	display: flex;
}

</style>
