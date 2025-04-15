<script lang="ts">
	import bgImage from '$lib/assets/bg-image.webp';
	import smBgImage from '$lib/assets/bg-image.sm.webp';
	import IconifyIcon from '@iconify/svelte';

	type ContactUsType = {
		icon: string
		link: string
	}

	let contactUs: ContactUsType[] = [
		{
			icon: 'cib:line',
			link: 'https://lin.ee/NW5iyIk'
		},
		{
			icon: 'skill-icons:instagram',
			link: 'https://www.instagram.com/chenwanglaw?igsh=MW0yY3pkbzlxN2lidQ%3D%3D&utm_source=qr'
		},
		{
			icon: 'devicon:facebook',
			link: 'https://www.facebook.com/share/1BfUcyRZki/?mibextid=wwXIfr&ref=xav_pl_fb_external_link_ios'
		}
	];
</script>

<div class="relative flex h-36 justify-center overflow-clip md:h-[480px]">
	<picture>
		<source media="(min-width: 768px)" srcset={bgImage}>
		<img
			alt="bg"
			class="absolute inset-0 z-0 h-36 w-full animate-[--brightness_3s_ease-in-out] object-cover brightness-75 md:h-[480px]"
			src={smBgImage}
		/>
	</picture>

	<div class="z-10 pt-4 md:pt-36 text-center">
		<p class="text-2xl md:text-4xl lg:text-6xl text-gray-300 md:text-6xl">勤睿國際法律事務所</p>

		<div class="fixed lg:relative flex flex-col md:mt-8 bottom-[3rem] lg:bottom-auto right-[1.5rem] lg:right-auto">
			<div class="flex flex-col lg:flex-row gap-6 lg:gap-16">
				{#each contactUs as item (item.link)}
					<a
						class="flex flex-row gap-4 lg:rounded lg:border lg:px-8 lg:py-4 w-fit border-yellow-100 items-center rounded-[50%] p-2 bg-white lg:bg-transparent shadow-xl lg:shadow-none"
						href={item.link}
						target="_blank">
						<IconifyIcon class="h-8 w-8 lg:h-10 lg:w-10 text-green-400" icon={item.icon} />
						<p class="hidden lg:block lg:text-yellow-100 lg:text-xl lg:text-3xl">諮詢</p>
					</a>
				{/each}
			</div>
		</div>
	</div>
</div>
