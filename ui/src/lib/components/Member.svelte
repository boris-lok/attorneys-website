<script lang="ts">
    import type {MemberPreview} from "$lib/models/Member";
    import SvelteMarkdown from "svelte-markdown";
    import {onMount} from "svelte";
    import {t} from "svelte-i18n";

    export let member: MemberPreview;
    let image: HTMLElement;

    onMount(() => {
        if ('file' in member && member.file) {
            const reader = new FileReader();
            reader.onload = () => {
                if (typeof reader.result === "string") {
                    image.setAttribute('src', reader.result);
                }
            }
            reader.readAsDataURL(member.file);
        }
    });
</script>

<div class="member-wrapper">
    <section>
        <img class="avatar-wrapper" bind:this={image} alt="{member.name}" src="" width="128" height="128"/>
    </section>
    <section>
        <p class="name-wrapper">{member.name}&nbsp;{$t('member.attorney.suffix')}</p>
    </section>
    <section>
        <div class="description-wrapper">
            <SvelteMarkdown source={member.description} />
        </div>
    </section>
</div>

<style lang="scss">
  .member-wrapper {
    display: flex;
    flex-direction: column;
    box-sizing: border-box;
    padding: 24px 16px;
    align-items: center;

    .avatar-wrapper {
      border-radius: 50%;
    }

    .name-wrapper {
      font-size: 20px;
    }

    .description-wrapper {
      border-radius: 4px;
      box-shadow: 0 0 10px 0 $grey;
      padding: 24px 16px;
    }
  }
</style>