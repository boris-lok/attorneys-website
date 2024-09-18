<script lang="ts">
    import type {MemberPreview} from "$lib/models/Member";
    import SvelteMarkdown from "svelte-markdown";
    import {t} from "svelte-i18n";

    export let member: MemberPreview;
    let image: HTMLElement;

    let hasImage = false;

    function handleMemberChanged(member: MemberPreview) {
        hasImage = false;
        if ('file' in member && member.file) {
            hasImage = true;
            const reader = new FileReader();
            reader.onload = () => {
                if (typeof reader.result === "string") {
                    image.setAttribute('src', reader.result);
                }
            }
            reader.readAsDataURL(member.file);
        }
    }

    $: handleMemberChanged(member);
</script>

<div class="member-wrapper">
    <section>
        <div class="avatar-wrapper">
            <img alt="{member.name}" bind:this={image} class:show={hasImage} height="128" src="" width="128"/>
            <span class="material-icon" class:show={!hasImage}>account_circle</span>
            <p>{member.name}&nbsp;{$t('member.attorney.suffix')}</p>
        </div>
    </section>
    <section>
        <div class="description-wrapper">
            <SvelteMarkdown source={member.description}/>
        </div>
    </section>
</div>

<style lang="scss">
  .member-wrapper {
    display: flex;
    flex-direction: column;
    padding: 1.5rem 1rem;
    align-items: center;
    gap: 1rem;

    section {
      width: 100%;
    }

    .avatar-wrapper {
      display: flex;
      flex-direction: column;
      gap: 1rem;
      align-items: center;

      img {
        display: none;
        border-radius: 50%;

        &.show {
          display: block;
        }
      }

      span {
        display: none;
        font-size: 128px;

        &.show {
          display: block;
        }
      }

      p {
        font-size: 20px;
        margin: 0;
      }
    }

    .description-wrapper {
      border-radius: 4px;
      box-shadow: 0 0 10px 0 $grey;
      padding: 24px 16px;
      width: 100%;
    }
  }

  @media (min-width: 768px) {
    .member-wrapper {
      flex-direction: row;

      section:nth-child(1) {
        width: 25%;
      }

      section:nth-child(2) {
        width: 75%;
        min-height: 560px;

        .description-wrapper {
          display: flex;
          flex-direction: column;
          box-shadow: none;
          height: 66vh;
          overflow-y: auto;
          justify-content: center;
          background-color: $white;
        }
      }
    }
  }
</style>