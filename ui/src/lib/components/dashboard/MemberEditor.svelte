<script lang="ts">
    import UploadImage from '$lib/components/common/UploadImage.svelte';
    import type { ImageData } from '$lib/types';
    import Input from '$lib/components/common/Input.svelte';
    import Textarea from '$lib/components/common/Textarea.svelte';
    import Markdown from '@magidoc/plugin-svelte-marked';
    import { MemberServices } from '$lib/services/member.service';
    import { startWithTap } from '$lib/utils';
    import { concatMap, finalize, of, tap } from 'rxjs';
    import Loading from '$lib/components/common/Loading.svelte';

    type InputProps = {
		id?: string
		name?: string
		description?: string
		avatarData?: ImageData
		seq?: number
	}

	let _in: InputProps = $props();

	let name = $state('');
	let description = $state('');
	let avatarData: ImageData | undefined = $state(undefined);
	let newAvatar: File | undefined = undefined;
	let errorMsg = $state('');

	$effect(() => {
		// init the state by props
		name = _in.name ?? '';
		description = _in.description ?? '';
		avatarData = _in.avatarData;
	});

	let isLoading = $state(false);

	// handles the avatar has been changed
	function onImageChanged(file: File | undefined) {
		newAvatar = file;
		console.log(newAvatar);
	}

	// handles the name has been changed
	function onNameChanged(
		e: Event & { currentTarget: EventTarget & HTMLInputElement }
	) {
		name = (e.target as HTMLInputElement).value.trim();
	}

	// handles the description has been changed
	function onDescriptionChanged(
		e: Event & { currentTarget: EventTarget & HTMLTextAreaElement }
	) {
		description = (e.target as HTMLTextAreaElement).value.trim();
	}

	// a function that validate the all data is correct
	function validate(data: {
		name: string | undefined
		description: string | undefined
	}) {
		return (
			data.name &&
			data.name.trim() !== '' &&
			data.description &&
			data.description.trim() !== ''
		);
	}

	// An handler handles the submit button has been clicked
	function onSaveClicked() {
		if (isLoading) {
			return;
		}

		if (!validate({ name: name, description: description })) {
			return;
		}

		MemberServices.save({
			...(_in.id === undefined ? {} : { id: _in.id }),
			name: name!,
			description: description!,
			seq: 0,
			language: 'zh'
		})
			.pipe(
				startWithTap(() => (isLoading = true)),
				concatMap((resp) => {
					if ('error' in resp && resp.error) {
						return of(resp);
					} else if (
						newAvatar &&
						newAvatar instanceof File &&
						'id' in resp
					) {
						console.log('saving avatar...');
						return MemberServices.saveAvatar(resp.id, newAvatar);
					}

					return of({ error: false });
				}),
				finalize(() => (isLoading = false)),
				tap((resp) => {
					if ('message' in resp) {
						console.error('Error saving content:', resp.message);
						errorMsg = 'We got an error when saving content.';
					}
				})
			)
			.subscribe();
	}
</script>

{#if isLoading}
	<Loading />
{:else}
	<div
		class="mb-2 flex rounded-lg bg-red-50 p-4 text-sm text-red-800 hidden [.show]:block"
		class:show={errorMsg!==''}
		role="alert"
	>
		<p class="w-full text-center">{errorMsg}</p>
	</div>
	<div class="relative flex flex-col gap-y-4 px-4 py-4 md:flex-row md:gap-x-4">
		<div class="flex-1">
			<div class="relative flex flex-col gap-4">
				<UploadImage imageData={avatarData} onChange={onImageChanged} />
				<Input
					hasError={errorMsg !== ''}
					label="Name"
					name="name"
					onInput={onNameChanged}
					type="text"
					value={name ?? ''}
				/>
				<Textarea
					label="Description"
					name="description"
					onInput={onDescriptionChanged}
					value={description ?? ''}
				></Textarea>
			</div>
		</div>
		<div class="flex-1">
			<p class="mb-2 block text-sm font-medium text-gray-900">Preview</p>
			<div
				class="prose block min-h-96 w-full rounded-lg bg-gray-100 px-4 py-4"
			>
				<p class="py-2 text-lg font-bold text-[var(--primary-color)]">
					{name}
				</p>
				<div class="prose">
					<Markdown source={description ?? ''}></Markdown>
				</div>
			</div>
		</div>
	</div>

	<div class="relative flex flex-row justify-center gap-x-4">
		<button
			class="block cursor-pointer rounded bg-blue-500 px-4 py-2 font-bold text-white hover:bg-blue-700 focus:outline-none disabled:cursor-auto disabled:bg-gray-500"
			disabled={name === '' || description === ''}
			onclick={onSaveClicked}
		>
			Save
		</button>
	</div>
{/if}