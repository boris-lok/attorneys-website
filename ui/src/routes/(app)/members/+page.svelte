<script lang="ts">
	import { onMount } from 'svelte';
	import type { SimpleMember } from '$lib/models/Member';
	import { Members } from '$lib/services';
	import { startWithTap } from '$lib/utils';
	import { finalize, tap } from 'rxjs';
	import Loading from '$lib/components/Loading.svelte';
	import { t } from 'svelte-i18n';
	import type { Language } from '$lib/models/Language';
	import Avatar from '$lib/components/Avatar.svelte';

	// The members data
	let members: SimpleMember[] = [];
	// The flag that indicates the page is loading
	let isLoading = false;
	// The language that we want to fetch
	let language: Language = 'zh';

	onMount(() => {
		Members.list(language)
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
					<a href="/members/{member.id}">
						{#if member.avatar}
							<Avatar avatar={member.avatar} regularSize={64} />
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

  h1 {
    text-align: center;
  }

  .member-card {
    border-radius: 0.25rem;
    box-shadow: 0 0 1rem 0 $grey;
    padding: 1.25rem;
    position: relative;

    .bg {
      position: absolute;
      top: 0;
      left: 0;
      width: 0;
      height: 100%;
      background: linear-gradient(90deg, rgba(234, 88, 12, 0.5) 0%, rgba(232, 232, 232, 0) 100%);;
      z-index: -1;
      transition: width .5s linear;
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

  @media (min-width: 768px) {
    .members-section {
      padding: 0 5%;
      margin: 0 auto;
      max-width: 1280px;
    }

    .members-list-section {
      flex-direction: row;
      flex-wrap: wrap;
      padding: 1.5rem 0;
      gap: 1rem;

      .member-card {
        min-width: 22rem;

        a {
          flex-direction: row;
          gap: 1rem;

          span.material-icon {
            margin-bottom: 0;
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
    }
  }
</style>