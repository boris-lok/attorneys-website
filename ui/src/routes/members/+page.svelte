<script lang="ts">
	import { onMount } from 'svelte';
	import type { SimpleMember } from '$lib/models/Member';
	import { Members } from '$lib/services';
	import { startWithTap } from '$lib/utils';
	import { finalize, tap } from 'rxjs';
	import Loading from '$lib/components/Loading.svelte';
	import { t } from 'svelte-i18n';

	let members: SimpleMember[] = [];
	let isLoading = false;

	onMount(() => {
		Members.list()
			.pipe(
				startWithTap(() => isLoading = true),
				finalize(() => isLoading = false),
				tap(e => members = e)
			)
			.subscribe();
	});
</script>

{#if isLoading}
	<Loading />
{:else}
	<div class="members-section">
		<h1>{$t('Members')}</h1>
		<div class="members-list-section">
			{#each members as member}
				<div class="member-card">
					<div class="bg"></div>
					<a href="/members/{member.member_id}">
						{#if member.avatar}
						{:else}
							<span class="material-icon">account_circle</span>
						{/if}
						<span class="name">
							<h3>{member.name}</h3>
							<p>{$t('member.attorney.suffix')}</p>
						</span>
					</a>
				</div>
			{/each}
		</div>
	</div>
{/if}


<style lang="scss">
  .members-list-section {
    display: flex;
    flex-direction: column;
    gap: 2rem;
    padding: 1.25rem 5%;
  }

  .member-card {
    border-radius: 0.25rem;
    box-shadow: 0 0 1rem 0 $grey;
    padding: 1.25rem 0;
    position: relative;

    .bg {
      position: absolute;
      top: 0;
      left: 0;
      width: 0;
      height: 100%;
      background: linear-gradient(90deg, rgba(234, 88, 12, 0.5) 0%, rgba(232, 232, 232, 0) 100%);;
      z-index: -1;
      transition: width 1s linear;
    }

    &:hover {
      .bg {
        width: 100%;
      }
    }

    a {
      text-decoration: none;
      color: $black;
      display: flex;
      flex-direction: column;
      align-items: center;

      span.material-icon {
        font-size: 4rem;
        margin-bottom: 0.5rem;
      }

      span.name {
        display: flex;
        flex-direction: row;
        align-items: flex-end;
        gap: 0.5rem;
        justify-content: center;

        h3 {
          font-weight: 700;
        }
      }
    }
  }
</style>