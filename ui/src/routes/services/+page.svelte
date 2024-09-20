<script lang="ts">
	import type { Service } from '$lib/models/Services';
	import { onMount } from 'svelte';
	import { Services } from '$lib/services';
	import { shuffle, startWithTap } from '$lib/utils';
	import { finalize, tap } from 'rxjs';
	import Loading from '$lib/components/Loading.svelte';
	import SvelteMarkdown from 'svelte-markdown';

	let loading = false;
	let data: Service[] = [];
	let imgs: string[] = [];


	onMount(() => {
		const images = import.meta.glob('$lib/assets/*_480.png', { eager: true });
		for (let key in Object.keys(images)) {
			let value = Object.values(images)[key];
			if (typeof value === 'object' && value != null && 'default' in value && typeof value.default == 'string') {
				imgs.push(value.default);
			}
		}

		Services.list()
			.pipe(
				startWithTap(() => loading = true),
				finalize(() => loading = false),
				tap(services => {
					data = services;
					while (data.length > imgs.length) {
						imgs = [...imgs, ...imgs];
					}
					imgs = shuffle(imgs);
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
				<h3>{service.title}</h3>
				<img src={imgs[i]} alt="{service.title}" />
				<SvelteMarkdown source={service.content} />
			</div>
		{/each}
	</div>
{/if}

<style lang="scss">
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

