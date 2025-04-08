<script lang="ts">
	import bgImage from '$lib/assets/bg-image.webp';
	import smBgImage from '$lib/assets/bg-image.sm.webp';
	import IconifyIcon from '@iconify/svelte';
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
		<p class="text-6xl text-gray-300 md:text-6xl">勤睿國際法律事務所</p>

		<div class="flex flex-col md:mt-8">

			<div class="flex flex-col lg:flex-row gap-16">
				<a href="https://lin.ee/NW5iyIk"
					 target="_blank"
					 class="flex flex-row gap-4 md:rounded md:border md:px-8 md:py-4 w-fit border-yellow-100">
					<IconifyIcon class="h-10 w-10 text-green-400" icon="cib:line" />
					<p class="text-yellow-100 text-xl md:text-3xl">諮詢</p>
				</a>

				<a href="https://www.instagram.com/chenwanglaw?igsh=MW0yY3pkbzlxN2lidQ%3D%3D&utm_source=qr"
					 target="_blank"
					 class="flex flex-row gap-4 md:rounded md:border md:px-8 md:py-4 w-fit border-yellow-100">
					<IconifyIcon class="h-10 w-10 text-green-400" icon="skill-icons:instagram" />
					<p class="text-yellow-100 text-xl md:text-3xl">諮詢</p>
				</a>

				<a href="https://www.facebook.com/share/1BfUcyRZki/?mibextid=wwXIfr&ref=xav_pl_fb_external_link_ios"
					 class="flex flex-row gap-4 md:rounded md:border md:px-8 md:py-4 w-fit border-yellow-100"
					 target="_blank">
					<IconifyIcon class="h-10 w-10 text-green-400" icon="devicon:facebook" />
					<p class="text-yellow-100 text-xl md:text-3xl">諮詢</p>
				</a>
			</div>
		</div>
	</div>
</div>
