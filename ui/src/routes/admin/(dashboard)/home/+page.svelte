<script lang="ts">
    import Markdown from '@magidoc/plugin-svelte-marked';
    import { HomeServices } from '$lib/services/home.service';
    import { startWithTap } from '$lib/utils';
    import { finalize, tap } from 'rxjs';
    import Icon from '@iconify/svelte';
    import Loading from '$lib/components/common/Loading.svelte';

    let id = $state('');
	// The content of home page
	let content = $state('');
	// The loading statue of retrieving the content from API.
	let isLoading = $state(false);

	// fetch the content from API.
	function fetchData() {
		HomeServices.list('zh')
			.pipe(
				startWithTap(() => (isLoading = true)),
				finalize(() => (isLoading = false)),
				tap((resp) => {
					if (resp.length === 0) {
						content = '';
					} else {
						content = resp[0].data.data;
						id = resp[0].id;
					}
				})
			)
			.subscribe({
				error: console.error
			});
	}

	$effect(() => {
		fetchData();
	});
</script>

{#if isLoading}
	<Loading />
{:else}
	<div
		class="relative mx-auto flex max-w-[var(--max-screen-width)] flex-col gap-y-8 px-16 py-16"
	>
		<div class="relative flex flex-row justify-end">
			{#if id !== ''}
				<a href="/admin/home/edit/{id}">
					<Icon icon="mingcute:edit-line" width="24" height="24" />
				</a>
			{:else}
				<a href="/admin/home/edit">
					<Icon icon="gridicons:create" width="24" height="24" />
				</a>
			{/if}
		</div>
		<div class="prose">
			<Markdown source={content}></Markdown>
		</div>
	</div>
{/if}