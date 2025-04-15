<script lang="ts">
	import { MemberServices } from '$lib/services/member.service';
	import { startWithTap } from '$lib/utils';
	import { finalize, tap } from 'rxjs';
	import type { Language, SimpleMember } from '$lib/types';
	import Image from '$lib/components/common/Image.svelte';
	import IconifyIcon from '@iconify/svelte';
	import Loading from '$lib/components/common/Loading.svelte';

	let members: SimpleMember[] = $state([]);
	let isLoading = $state(false);
	let lang: Language = 'zh';

	$effect(() => {
		MemberServices.list(lang)
			.pipe(
				startWithTap(() => (isLoading = true)),
				finalize(() => (isLoading = false)),
				tap((resp) => {
					members = resp;
				})
			)
			.subscribe({ error: console.error });
	});
</script>

{#if isLoading}
	<Loading />
{:else}
	<div class="relative flex flex-col md:flex-row md:items-center mt-8 md:mt-16">
		<div class="relative flex flex-col gap-4 w-full">
			<p
				class="px-4 text-4xl font-bold text-[var(--primary-color)] md:px-8 lg:px-16 w-full text-center"
			>
				本所成員
			</p>
			<div
				class="relative flex flex-col gap-8 px-4 md:flex-row md:px-24 lg:px-48 mt-4 md:justify-center"
			>
				{#each members as member}
					<a href="/members/{member.id}">
						<div
							class="flex h-36 w-full flex-row items-center gap-6 rounded px-4 py-4 shadow-md md:w-84 lg:w-96"
						>
							{#if member.avatar}
								<div class="h-24 w-24">
									<Image
										alt={member.name}
										image={member.avatar}
									/>
								</div>
							{:else}
								<IconifyIcon
									icon="radix-icons:avatar"
									class="h-24 w-24"
								/>
							{/if}
							<p class="text-2xl">{member.name}</p>
						</div>
					</a>
				{/each}
			</div>
		</div>
	</div>

{/if}