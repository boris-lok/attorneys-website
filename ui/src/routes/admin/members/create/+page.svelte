<script lang="ts">
    import Input from "$lib/components/Input.svelte";
    import {t} from 'svelte-i18n';
    import LoadingButton from "$lib/components/LoadingButton.svelte";
    import TextArea from "$lib/components/TextArea.svelte";
    import UploadImage from "$lib/components/UploadImage.svelte";
    import type {MemberPreview} from "$lib/models/Member";
    import Member from "$lib/components/Member.svelte";

    let m: MemberPreview | undefined;
    let isPreview = false;
    let name = "";
    let description = "";
    let avatar: File | undefined;

    // Handle image changed
    function onImageChanged(e: CustomEvent) {
        avatar = e.detail.file;
        console.log(`avatar has been changed: ${avatar}`);
    }

    // Handle name changed
    function onNameChanged(e: Event) {
        const target = e.target as HTMLInputElement;
        name = target.value.trim();
    }

    // Handle description changed
    function onDescriptionChanged(e: Event) {
        const target = e.target as HTMLTextAreaElement;
        description = target.value.trim();
    }

    // Check the name and description is not empty
    function validate() {
        return name.trim() !== "" && description !== "";
    }

    // Handle preview button clicked and show the data
    function onPreviewClicked() {
        if (validate()) {
            isPreview = true;
            m = {
                name: name,
                description: description,
                file: avatar,
            }
            console.log(`${JSON.stringify(m)}`);
        }
    }
</script>

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