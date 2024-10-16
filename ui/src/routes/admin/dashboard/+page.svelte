<script>
	import HomeManager from '$lib/components/dashboard/HomeManager.svelte';
	import ContactUsManager from '$lib/components/dashboard/ContactUsManager.svelte';
	import ServiceManager from '$lib/components/dashboard/ServiceManager.svelte';
	import ArticleManager from '$lib/components/dashboard/ArticleManager.svelte';
	import MemberManager from '$lib/components/dashboard/MemberManager.svelte';
</script>

<div class="dashboard-wrapper">
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


<style lang="scss">
  .dashboard-wrapper {
    display: grid;
    grid-template:
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

  @media (min-width: 768px) {
    .dashboard-wrapper {
      grid-template: 'home contact-us'
		'services services'
		'members members'
		'articles articles';
    }
  }
</style>