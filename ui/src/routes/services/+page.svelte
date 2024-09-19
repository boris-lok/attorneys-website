<script lang="ts">
    import {t} from "svelte-i18n";
    import type {Service} from "$lib/models/Services";
    import {onMount} from "svelte";
    import {Services} from "$lib/services";
    import {startWithTap} from "$lib/utils";
    import {finalize, tap} from "rxjs";
    import Loading from "$lib/components/Loading.svelte";
    import SvelteMarkdown from "svelte-markdown";

    let loading = false;
    let data: Service[] = [];

    onMount(() => {
        Services.list()
            .pipe(
                startWithTap(() => loading = true),
                finalize(() => loading = false),
                tap(services => data = services)
            )
            .subscribe()
    })
</script>

{#if loading}
    <Loading/>
{:else}
    <div class="services-wrapper">
        {#each data as service}
            <div class="service-section">
                <h3>{service.title}</h3>
                <div class="divider"></div>
                <SvelteMarkdown source={service.content}/>
            </div>
        {/each}
    </div>
{/if}

<style lang="scss">
  .service-section {
    h3 {
      border-bottom: 1px solid $grey;
      margin-bottom: 1rem;
    }
  }

  @media (min-width: 768px) {
    .services-wrapper {
      padding-top: 8rem;
      display: flex;
      flex-direction: column;
      gap: 2rem;
    }

    .service-section {
      display: flex;
      flex-direction: row;
      box-shadow: 0 0 1rem 0 $grey;
      padding: 1.25rem 2.5rem;
      border-radius: 0.25rem;

      h3 {
        min-width: 15rem;
        width: 15rem;
        border-bottom: none;
        border-right: 1px solid $grey;
        margin-bottom: 0;
        display: flex;
        justify-content: center;
        align-items: center;
      }
    }
  }
</style>

