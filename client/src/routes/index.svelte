<script context="module">
	export async function load({ session }) {
		console.log(session.user);
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
</script>

<div class="content">
	<p>Very Rezsi, So Profit!</p>
</div>

<style lang="scss">
	.content {
		display: flex;
		flex-direction: column;
		align-items: center;
	}
</style>
