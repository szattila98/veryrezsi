<script context="module">
	export async function load({ session }) {
		if (!session?.user) {
			return {
				status: 302,
				redirect: '/login',
			};
		}
		return {
			props: {
				user: session.user,
			},
		};
	}
</script>

<script lang="ts">
	export let user;

	// If you want to get user without the ugly context="module" block
	// import { session } from '$app/stores';
	// export let user = $session.user;
</script>

<div class="content">
	<h1>Very Rezsi, So Profit!</h1>
	<p>Your email: {user.email}</p>
</div>

<style lang="scss">
	.content {
		display: flex;
		flex-direction: column;
		align-items: center;
	}
</style>
