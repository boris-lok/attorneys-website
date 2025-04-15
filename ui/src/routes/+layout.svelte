<script lang="ts">
	import '../app.css';

	let { children } = $props();
</script>

<div class="relative">
	{@render children()}
</div>
