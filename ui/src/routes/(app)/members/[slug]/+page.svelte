<script lang="ts">
	import { page } from '$app/stores';
	import { onMount } from 'svelte';
	import { Members } from '$lib/services';
	import { startWithTap } from '$lib/utils';
	import { finalize, tap } from 'rxjs';
	import Loading from '$lib/components/Loading.svelte';
	import { t } from 'svelte-i18n';
	import SvelteMarkdown from 'svelte-markdown';
	import type { MemberData } from '$lib/models/Member';
	import type { Language } from '$lib/models/Language';
	import Avatar from '$lib/components/Avatar.svelte';

	let id = $page.params.slug;
	let language: Language = 'zh';
	let isLoading = false;
	let member: MemberData | null = null;

	onMount(() => {
		Members.retrieve(id, language)
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
		{#if member && member.avatar}
			<div class="avatar-section">
				<Avatar avatar={member.avatar} />
			</div>
		{/if}
		{#if member}
			<div class="member-detail-section add-margin-to-listview">
				<h1>{member.data.name}</h1>
				<SvelteMarkdown source={member.data.description} />
			</div>
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

    .avatar-section {
      margin: 0 auto;
    }
  }

  @media (min-width: 768px) {
    .member-section {
      max-width: 1024px;
      padding: 0 10% 1.5rem 10%;
      margin: 0 auto;
      display: flex;
      flex-direction: row-reverse;
      justify-content: space-between;

      .avatar-section {
        margin: 0;
      }
    }
  }
</style>