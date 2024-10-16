<script lang="ts">
	import { page } from '$app/stores';
	import Loading from '$lib/components/Loading.svelte';
	import { t } from 'svelte-i18n';
	import CreateOrEditMember from '$lib/components/dashboard/CreateOrEditMember.svelte';
	import type { MemberData } from '$lib/models/Member';
	import { onMount } from 'svelte';
	import { Members } from '$lib/services';
	import type { Language } from '$lib/models/Language';
	import { startWithTap } from '$lib/utils';
	import { finalize, tap } from 'rxjs';

	// isLoading is a flag that indicates we are loading a resource from API.
	let isLoading = false;
	// The ID of the Home data resource
	let id = $page.params.slug;
	// The member data
	let data: MemberData | null = null;
	// The language of the resource
	let language: Language = 'zh';

	onMount(() => {
		Members.retrieve(id, language)
			.pipe(
				startWithTap(() => isLoading = true),
				finalize(() => isLoading = false),
				tap(e => {
					data = e;
				})
			)
			.subscribe({
				error: console.error
			});
	});

</script>

{#if isLoading}
	<Loading />
{:else if data}
	<CreateOrEditMember id={data.id} name={data.data.name} description={data.data.description}
											avatarData={data.avatar ?? null} />
{:else}
	<p>{$t('no_data_available')}</p>
{/if}