<script lang="ts">
    import Input from "$lib/components/Input.svelte";
    import {t} from 'svelte-i18n';
    import LoadingButton from "$lib/components/LoadingButton.svelte";
    import TextArea from "$lib/components/TextArea.svelte";
</script>

<main>
    <section>
        <div class="wrapper">
            <div class="form-wrapper">
                <Input label={$t('create.members.name')} name="name"/>
                <TextArea label={$t('create.members.description')}/>
                <div class="btn-container">
                    <LoadingButton text={$t('create.members.create_btn')} classname="primary-blue"/>
                    <LoadingButton text={$t('create.members.cancel_btn')} classname="primary-red"/>
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
      display: flex;
      flex-direction: column;
      gap: 24px;
      padding: 24px 16px;
      border-radius: 4px;
      box-shadow: 0 0 16px 0 $grey;
      background-color: $white;

      .btn-container {
        display: flex;
        flex-direction: row;
        column-gap: 12px;
        width: 100%;
        justify-content: center;
      }
    }
  }
</style>