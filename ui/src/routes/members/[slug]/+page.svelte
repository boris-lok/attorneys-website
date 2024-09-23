<script lang="ts">
	import { page } from '$app/stores';
	import { onMount } from 'svelte';
	import { Members } from '$lib/services';
	import type { MemberDetail } from '$lib/models/Member';
	import { startWithTap } from '$lib/utils';
	import { finalize, tap } from 'rxjs';
	import Loading from '$lib/components/Loading.svelte';
	import { t } from 'svelte-i18n';
	import SvelteMarkdown from 'svelte-markdown';

	let member_id = $page.params.slug;
	let isLoading = false;
	let member: MemberDetail;

	onMount(() => {
		Members.get(member_id)
			.pipe(
				startWithTap(() => isLoading = true),
				finalize(() => isLoading = false),
				tap(e => member = e)
			)
			.subscribe();
	});
</script>

{#if isLoading}
	<Loading />
{:else}
	<div class="member-section">
		{#if member}
			<h1>{member.name}</h1>
			<SvelteMarkdown source={member.content} />
		{:else}
			<p>{$t('no_member_message')}</p>
		{/if}
	</div>
{/if}


<style lang="scss">
  .member-section {
    display: flex;
    flex-direction: column;
    padding: 0 2.5rem 1.25rem 10%;

    p {
      text-align: center;
    }
  }

  @media (min-width: 768px) {
    .member-section {
      max-width: 1024px;
      padding: 0 10% 1.5rem 10%;
      margin: 0 auto;
    }
  }
</style>