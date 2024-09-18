<script lang="ts">
    import Input from "$lib/components/Input.svelte";
    import {t} from 'svelte-i18n';
    import LoadingButton from "$lib/components/LoadingButton.svelte";
    import TextArea from "$lib/components/TextArea.svelte";
    import UploadImage from "$lib/components/UploadImage.svelte";
    import type {MemberPreview} from "$lib/models/Member";
    import Member from "$lib/components/Member.svelte";
    import {finalize} from "rxjs";
    import {Members} from "$lib/services";
    import {startWithTap} from "$lib/utils";

    let m: MemberPreview | undefined;
    let isPreview = false;
    let name = "";
    let description = "";
    let avatar: File | undefined;
    let loading = false;

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
    function onPreviewBtnClicked() {
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

    // Handle back button clicked and hide the preview section
    function onBackBtnClicked() {
        isPreview = false;
    }

    function onCreateBtnClicked() {
        if (validate() && !loading) {
            Members.create({name: name, description: description}, 'zh')
                .pipe(
                    startWithTap(() => loading = true),
                    finalize(() => loading = false),
                )
                .subscribe({
                    next: (res) => {
                        if (avatar) {
                            Members.uploadAvatar(res.member_id, avatar)
                                .pipe(
                                    startWithTap(() => loading = true),
                                    finalize(() => loading = false),
                                ).subscribe({
                                error: e => {
                                    console.error(`Can't upload an avatar, got an error: ${e}`)
                                }
                            })
                        }
                    },
                    error: e => {
                        console.error(`Can't create a member, got an error: ${e}`)
                    }
                })
        }
    }
</script>

<section class:hidden={isPreview}>
    <div class="wrapper">
        <div class="form-wrapper">
            <UploadImage on:change={onImageChanged}/>
            <div class="base-information-wrapper">
                <Input label={$t('create.members.name')} name="name" on:input={onNameChanged}/>
                <TextArea label={$t('create.members.description')} on:input={onDescriptionChanged}/>
                <div class="btn-container">
                    <LoadingButton classname="primary-blue" on:click={onCreateBtnClicked}
                                   text={$t('create.members.create_btn')}/>
                    <LoadingButton classname="primary-orange" on:click={onPreviewBtnClicked}
                                   text={$t('create.members.preview_btn')}/>
                    <LoadingButton classname="primary-red" text={$t('create.members.cancel_btn')}/>
                </div>
            </div>
        </div>
    </div>
</section>
{#if isPreview && m}
    <div class="preview-section">
        <Member member={m}/>
        <div class="btn-container">
            <LoadingButton text={$t('create.members.create_btn')} classname="primary-blue"
                           on:click={onCreateBtnClicked}/>
            <LoadingButton text={$t('create.members.back_btn')} classname="primary-red" on:click={onBackBtnClicked}/>
        </div>
    </div>
{/if}

<style lang="scss">
  section.hidden {
    display: none;
  }

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


      }
    }
  }

  .preview-section {
    display: inline-block;
    padding: 2rem 0;
  }

  .btn-container {
    display: flex;
    flex-direction: row;
    column-gap: 12px;
    width: 100%;
    justify-content: center;
  }
</style>