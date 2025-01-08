<script lang="ts">
	import Markdown from '@magidoc/plugin-svelte-marked';

	let content = `
# Setup
Here are the steps to set up svelte-marked plugin
1. Install it
2. Enjoy
    - If you enjoyed, make sure to star the repo!
`;
</script>

<div class="prose">
	<Markdown source={content}></Markdown>
</div>