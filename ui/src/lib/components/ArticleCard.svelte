<script lang="ts">
	type InputProps = {
		id: string
		title: string
		createdAt: string
	}

	let article: InputProps = $props();
</script>

<a
	class="group flex flex-col bg-white rounded-lg shadow hover:shadow-xl p-6 transition duration-200 cursor-pointer justify-around"
	href="/articles/{article.id}">
	<h2 class="text-xl font-semibold text-gray-800 group-hover:text-[var(--primary-color)] transition">
		{article.title}
	</h2>

	<p class="text-gray-500 text-xs mt-4">
		發佈時間: {article.createdAt}
	</p>
</a>
