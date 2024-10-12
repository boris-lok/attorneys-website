<script lang="ts">
	import { page } from '$app/stores';
	import { onMount } from 'svelte';
	import { Services } from '$lib/services';
	import { startWithTap } from '$lib/utils';
	import { finalize, tap } from 'rxjs';
	import Loading from '$lib/components/Loading.svelte';
	import type { Language } from '$lib/models/Language';
	import { t } from 'svelte-i18n';
	import type { ServiceData } from '$lib/models/Services';
	import CreateOrEditService from '$lib/components/dashboard/CreateOrEditService.svelte';

	// isLoading is a flag that indicates we are loading a resource from API.
	let isLoading = false;
	// The ID of the Home data resource
	let id = $page.params.slug;
	// content is the actual content of the Markdown document.
	let service: ServiceData | null = null;
	let language: Language = 'zh';

	onMount(() => {
		Services.retrieve(id, language)
			.pipe(
				startWithTap(() => isLoading = true),
				finalize(() => isLoading = false),
				tap(e => service = e)
			).subscribe({ error: console.error });
	});
</script>

{#if isLoading}
	<Loading />
{:else if service}
	<CreateOrEditService title={service.data.title} data={service.data.data} id={id} />
{:else}
	<p>{$t('no_data_available')}</p>
{/if}