<script lang="ts">
    import Input from "$lib/components/Input.svelte";
    import {t} from 'svelte-i18n';
    import LoadingButton from "$lib/components/LoadingButton.svelte";
    import TextArea from "$lib/components/TextArea.svelte";
    import UploadImage from "$lib/components/UploadImage.svelte";

    function onImageChanged(e: CustomEvent) {
        console.log(e.detail.file);
    }
</script>

<main>
    <section>
        <div class="wrapper">
            <div class="form-wrapper">
                <UploadImage on:change={onImageChanged}/>
                <div class="base-information-wrapper">
                    <Input label={$t('create.members.name')} name="name"/>
                    <TextArea label={$t('create.members.description')}/>
                    <div class="btn-container">
                        <LoadingButton text={$t('create.members.create_btn')} classname="primary-blue"/>
                        <LoadingButton text={$t('create.members.cancel_btn')} classname="primary-red"/>
                    </div>
                </div>
            </div>
        </div>
    </section>
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