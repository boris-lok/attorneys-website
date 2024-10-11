<script lang="ts">
	import { page } from '$app/stores';
	import Loading from '$lib/components/Loading.svelte';
	import { onMount } from 'svelte';
	import { Contacts } from '$lib/services';
	import type { Language } from '$lib/models/Language';
	import { startWithTap } from '$lib/utils';
	import { finalize, tap } from 'rxjs';
	import type { ContactData } from '$lib/models/ContactUs';
	import CreateOrEditContact from '$lib/components/dashboard/CreateOrEditContact.svelte';

	// isLoading is a flag that indicates we are loading a resource from API.
	let isLoading = false;

	// The ID of the Home data resource
	let id = $page.params.slug;

	// The language of the data resource
	let language: Language = 'zh';

	// The contact data retrieved from the API.
	let contactData: ContactData | null = null;


	onMount(() => {
		Contacts.retrieve(id, language)
			.pipe(
				startWithTap(() => isLoading = true),
				finalize(() => isLoading = false),
				tap(e => {
					contactData = e;
				})
			).subscribe();
	});

</script>

{#if isLoading}
	<Loading />
{:else if contactData}
	<CreateOrEditContact id={id} address={contactData.data.address} phone={contactData.data.phone}
											 email={contactData.data.email} />
{/if}
