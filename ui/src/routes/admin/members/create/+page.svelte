<script lang="ts">
    import Input from "$lib/components/Input.svelte";
    import {t} from 'svelte-i18n';
    import LoadingButton from "$lib/components/LoadingButton.svelte";
    import TextArea from "$lib/components/TextArea.svelte";
    import UploadImage from "$lib/components/UploadImage.svelte";
    import type {MemberPreview} from "$lib/models/Member";
    import Member from "$lib/components/Member.svelte";

    let member: Partial<MemberPreview> = {};
    let m: MemberPreview | undefined;
    let isPreview = false;

    function onImageChanged(e: CustomEvent) {
        member = {
            ...member,
            file: e.detail.file,
        }
        console.log(e.detail.file);
    }

    function onNameChanged(e: Event) {
        const target = e.target as HTMLInputElement;
        member = {
            ...member,
            name: target.value.trim(),
        }
    }

    function onDescriptionChanged(e: Event) {
        const target = e.target as HTMLTextAreaElement;
        member = {
            ...member,
            description: target.value.trim(),
        }
    }

    function onPreviewClicked() {
        if (member.name && member.description && member.file) {
            isPreview = true;
            const name = member.name;
            const description = member.description;
            const file = member.file;

            m = {
                name: name,
                description: description,
                file: file,
            }
        }
    }
</script>

<main>
    <!--{#if !isPreview}-->
    <section>
        <div class="wrapper">
            <div class="form-wrapper">
                <UploadImage on:change={onImageChanged}/>
                <div class="base-information-wrapper">
                    <Input label={$t('create.members.name')} name="name" on:input={onNameChanged}/>
                    <TextArea label={$t('create.members.description')} on:input={onDescriptionChanged}/>
                    <div class="btn-container">
                        <LoadingButton text={$t('create.members.create_btn')} classname="primary-blue"/>
                        <LoadingButton text={$t('create.members.preview_btn')} classname="primary-orange"
                                       on:click={onPreviewClicked}/>
                        <LoadingButton text={$t('create.members.cancel_btn')} classname="primary-red"/>
                    </div>
                </div>
            </div>
        </div>
    </section>
    <!--{/if}-->
    {#if isPreview && m}
        <div>
            <Member member={m}/>
        </div>
    {/if}
</main>

<style lang="scss">
  .wrapper {
    width: 100%;
    padding: 24px 16px;
    box-sizing: border-box;

    .form-wrapper {
      width: 100%;
      display: flex;
      flex-direction: column;
      gap: 24px;
      border-radius: 4px;
      box-shadow: 0 0 16px 0 $grey;
      background-color: $white;
      padding: 24px 16px;
      box-sizing: border-box;

      .base-information-wrapper {
        display: flex;
        flex-direction: column;
        gap: 24px;

        .btn-container {
          display: flex;
          flex-direction: row;
          column-gap: 12px;
          width: 100%;
          justify-content: center;
        }
      }
    }
  }
</style>