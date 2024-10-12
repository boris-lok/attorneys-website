<script lang="ts">
	import { onMount } from 'svelte';
	import { Services } from '$lib/services';
	import { shuffle, startWithTap } from '$lib/utils';
	import { finalize, tap } from 'rxjs';
	import Loading from '$lib/components/Loading.svelte';
	import SvelteMarkdown from 'svelte-markdown';
	import type { Language } from '$lib/models/Language';
	import type { ServiceData } from '$lib/models/Services';

	// isLoading is a flag that indicates we are loading a resource from API.
	let loading = false;
	// language is the language that we want to load
	let language: Language = 'zh';
	// The resource data
	let data: ServiceData[] = [];
	// The images that we want to display
	let images: string[] = [];


	onMount(() => {
		const imported = import.meta.glob('$lib/assets/*_480.png', { eager: true });
		for (let key in Object.keys(imported)) {
			let value = Object.values(imported)[key];
			if (typeof value === 'object' && value != null && 'default' in value && typeof value.default == 'string') {
				images.push(value.default);
			}
		}

		Services.list(language)
			.pipe(
				startWithTap(() => loading = true),
				finalize(() => loading = false),
				tap(services => {
					data = services;
					while (data.length > images.length) {
						images = [...images, ...images];
					}
					images = shuffle(images);
				})
			)
			.subscribe();
	});
</script>

{#if loading}
	<Loading />
{:else}
	<div class="services-wrapper">
		{#each data as service, i}
			<div class="service-section" class:even={i % 2 === 0}>
				<h3>{service.data.title}</h3>
				<img src={images[i]} alt="{service.data.title}" />
				<div class="add-margin-to-listview">
					<SvelteMarkdown source={service.data.data} />
				</div>
			</div>
		{/each}
	</div>
{/if}

<style lang="scss">
  .services-wrapper {
    padding: 1rem 5%;
  }

  .service-section {
    position: relative;

    h3 {
      border-bottom: 1px solid $grey;
      margin-bottom: 1rem;
    }

    img {
      display: none;
    }
  }

  @media (min-width: 768px) {
    .services-wrapper {
      display: flex;
      flex-direction: column;
      gap: 2rem;
      padding: 0;
      max-width: 1024px;
      margin: 0 auto;
    }

    .service-section {
      display: flex;
      flex-direction: column;
      box-shadow: 0 0 1rem 0 $grey;
      padding: 1.25rem 2.5rem 1.25rem 24rem;
      border-radius: 0.25rem;
      min-height: 280px;
      justify-content: center;
      overflow: clip;
      background: white;

      img {
        display: block;
        position: absolute;
        top: 0;
        left: 0;
        width: 22rem;
        max-width: 22rem;
        height: 100%;
        max-height: 100%;
      }

      h3 {
        min-width: 15rem;
        width: 15rem;
        border-bottom: none;
        margin-bottom: 0;
        color: $deep-blue;
        font-weight: 700;
        font-size: 2rem;
      }

      &.even {
        padding: 1.25rem 24rem 1.25rem 1.25rem;

        img {
          left: unset;
          right: 0;
        }

        h3 {
          color: $deep-orange;
        }
      }
    }
  }
</style>

