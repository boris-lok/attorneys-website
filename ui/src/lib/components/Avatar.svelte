<script lang="ts">
	import type { AvatarData } from '$lib/models/Member';

	export let avatar: AvatarData;

	const debug = import.meta.env.VITE_DEBUG === 'true';

	let largeImage = avatar.large_image;
	let smallImage = avatar.small_image;

	if (debug === true) {
		largeImage = `http://localhost/images/${avatar.large_image}`;
		smallImage = `http://localhost/images/${avatar.small_image}`;
	}
</script>

<div class="avatar">
	<picture>
		<source media="(min-width: 768px)" srcset={largeImage}>
		<img alt="" src={smallImage}>
	</picture>
</div>