<script lang="ts">
	import Paper, { Title, Content } from '@smui/paper';

	import type { user } from '$shared/domain';

	export let user: user;
</script>

<div id="userDetails">
	<div id="avatar">
		<img id="avatarImage" src="images/avatar_placeholder.svg" alt="avatar" />
	</div>
	<Paper id="detailsPaper" color="primary" variant="outlined">
		<Content>
			<div id="username">
				<div class="label">Username</div>
				<div>{user.username}</div>
			</div>
			<div id="email">
				<div class="label">Email</div>
				<div>{user.email}</div>
			</div>
		</Content>
	</Paper>
</div>

<style lang="scss">
	@use '../../style/app.scss';

	#userDetails {
		display: inline-block;
		position: relative;
		margin: 2rem;
		align-items: center;
	}

	#avatarImage {
		width: 8rem;
		top: -2rem;
		left: -5rem;
		position: absolute;
	}

	#username,
	#email,
	#avatar {
		min-width: 25vw;
		display: flex;
		justify-content: space-between;
		margin: 1rem;
	}

	.label {
		font-size: 1.5rem;
		margin: 0 2rem 1rem 2rem;
		text-decoration: underline;
		text-decoration-color: app.$main;
		text-decoration-thickness: 0.2rem;
	}
</style>
