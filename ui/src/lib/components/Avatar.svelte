<script lang="ts">
	import type { AvatarData } from '$lib/models/Member';

	export let avatar: AvatarData | string;

	const debug = import.meta.env.VITE_DEBUG === 'true';

	let largeImage = typeof avatar !== 'string' ? avatar.large_image : '';
	let smallImage = typeof avatar !== 'string' ? avatar.small_image : avatar;

	if (debug === true) {
		largeImage = `http://localhost/images/${largeImage}`;
		smallImage = `http://localhost/images/${smallImage}`;
	}
</script>

<div class="avatar">
	<picture>
		{#if typeof avatar !== 'string'}
			<source media="(min-width: 768px)" srcset={largeImage} width="256" height="256">
		{/if}
		<img alt="" src={smallImage} width="64" height="64">
	</picture>
</div>

<style lang="scss">
	.avatar {
		overflow: clip;

		img {
			border-radius: 50%;
		}
	}
</style>