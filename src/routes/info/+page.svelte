
<script lang="ts">
	import AppBounds from '$lib/components/AppBounds.svelte';
	import SshConnectionVerifier from '$lib/components/files/SshConnectionVerifier.svelte';
	import { Button } from '$lib/shadcn/ui/button';
</script>

<svelte:head>
	<title>Info</title>
</svelte:head>

<h1 class="text-3xl lg:text-6xl mb-14">
	This project has been made using
</h1>

<AppBounds>
	<div class="bg-gradient rounded-2xl">
		<div class="flex flex-col gap-5 p-10 overflow-hidden bg-black/80">
			<Button class="rounded-2xl"
							href="https://svelte.dev/">Svelte</Button>
			<Button class="rounded-2xl bg-accent-foreground hover:bg-accent-foreground/80"
							href="https://www.typescriptlang.org/">Typescript</Button>
			<Button class="rounded-2xl "
							href="https://kit.svelte.dev/">Sveltekit</Button>
			<Button class="rounded-2xl bg-accent-foreground hover:bg-accent-foreground/80"
							href="https://www.prisma.io/docs/orm">Prisma ORM</Button>
			<Button class="rounded-2xl "
							href="https://tailwindcss.com/">Tailwind</Button>
			<Button class="rounded-2xl bg-accent-foreground hover:bg-accent-foreground/80"
							href="https://github.com/colinhacks/zod">Zod</Button>
			<Button class="rounded-2xl"
							href="https://www.shadcn-svelte.com/docs/components">Shadcn-svelte</Button>
			<Button class="rounded-2xl bg-accent-foreground hover:bg-accent-foreground/80"
							href="https://gradient.page/tools/shadcn-ui-theme-generator">Shadcn UI Theme generator</Button>
			<Button class="rounded-2xl "
							href="https://www.formsnap.dev/docs/introduction">Formsnap</Button>
		</div>
	</div>
</AppBounds>