
<script lang="ts">
	import AppBounds from '$lib/components/AppBounds.svelte';
	import { Button } from '$lib/shadcn/ui/button';
	import H1 from "$lib/components/H1.svelte";
	import Dock from "$lib/components/Dock.svelte";
	import { env } from '$env/dynamic/public';
	import Meta from '$lib/components/Meta.svelte';
	
	const links = [
		{ name: 'Svelte', url: 'https://svelte.dev/' },
		{ name: 'Typescript', url: 'https://www.typescriptlang.org/' },
		{ name: 'Sveltekit', url: 'https://kit.svelte.dev/' },
		{ name: 'Prisma ORM', url: 'https://www.prisma.io/docs/orm' },
		{ name: 'Tailwind', url: 'https://tailwindcss.com/' },
		{ name: 'Zod', url: 'https://github.com/colinhacks/zod' },
		{ name: 'Shadcn-svelte', url: 'https://www.shadcn-svelte.com/docs/components' },
		{ name: 'Shadcn UI Theme generator', url: 'https://gradient.page/tools/shadcn-ui-theme-generator' },
		{ name: 'Formsnap', url: 'https://www.formsnap.dev/docs/introduction' },
	];
</script>

<Meta pageTitle="Info"/>

<AppBounds>
	<Dock>
		<H1>This project has been made with:</H1>
		<div class="grid sm:grid-cols-3 gap-4">
			{#each links as {url, name}, i}
				<Button class="rounded-xl data-[accented=true]:bg-accent-foreground data-[accented=true]:hover:bg-accent-foreground/80"
						data-accented={i % 2 === 0? 'true' : 'false'}
						href={url}>{name}</Button>
			{/each}
		</div>
	</Dock>
</AppBounds>