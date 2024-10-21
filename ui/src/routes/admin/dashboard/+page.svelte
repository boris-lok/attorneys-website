<script>
	import HomeManager from '$lib/components/dashboard/HomeManager.svelte';
	import ContactUsManager from '$lib/components/dashboard/ContactUsManager.svelte';
	import ServiceManager from '$lib/components/dashboard/ServiceManager.svelte';
	import ArticleManager from '$lib/components/dashboard/ArticleManager.svelte';
	import MemberManager from '$lib/components/dashboard/MemberManager.svelte';
	import { Users } from '$lib/services';
	import { startWithTap } from '$lib/utils';
	import { finalize } from 'rxjs';
	import { goto } from '$app/navigation';
	import Loading from '$lib/components/Loading.svelte';
	import { user } from '../../../stores/userStore';

	let isLoading = false;

	function onLogoutButtonClicked() {
		Users.logout()
			.pipe(
				startWithTap(() => isLoading = true),
				finalize(() => isLoading = false)
			)
			.subscribe(
				{
					next: () => {
						user.remove();
						goto('/login');
					},
					error: console.error
				}
			);
	}
</script>

{#if isLoading}
	<Loading />
{:else}
	<div class="dashboard-wrapper">
		<div class="function-tools">
			<button type="button" class="btn" on:click={onLogoutButtonClicked}>
				<span class="material-icon">logout</span>
				<span>Logout</span>
			</button>
		</div>
		<div class="home-section">
			<HomeManager />
		</div>
		<div class="contact-us-section">
			<ContactUsManager />
		</div>
		<div class="service-section">
			<ServiceManager />
		</div>
		<div class="member-section">
			<MemberManager />
		</div>
		<div class="article-section">
			<ArticleManager />
		</div>
	</div>
{/if}

<style lang="scss">
  .dashboard-wrapper {
    display: grid;
    grid-template: 'function-tools'
	'home'
	'contact-us'
	'services'
	'members'
	'articles';
    gap: 1rem;
  }

  .home-section {
    grid-area: home;
  }

  .contact-us-section {
    grid-area: contact-us;
  }

  .service-section {
    grid-area: services;
  }

  .member-section {
    grid-area: members;
  }

  .article-section {
    grid-area: articles;
  }

  .function-tools {
    grid-area: function-tools;
    display: flex;
    flex-direction: row;
    justify-content: flex-end;
    padding: 0.5rem 1rem;
    background-color: rgba(0, 0, 0, 0.5);
    backdrop-filter: blur(2rem);

    .btn {
      border: 0;
      padding: 0.5rem 1rem;
      background-color: transparent;
      color: $white;
      cursor: pointer;
      display: flex;
      flex-direction: row;
      column-gap: 0.25rem;
      align-items: center;

      span {
        &.material-icon {
          font-size: 1.25rem;
        }
      }
    }
  }

  @media (min-width: 768px) {
    .dashboard-wrapper {
      grid-template: 'function-tools function-tools'
			'home contact-us'
			'services services'
			'members members'
			'articles articles';
    }
  }
</style>