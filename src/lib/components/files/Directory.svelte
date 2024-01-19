<script lang="ts">
	import { Skeleton } from '$lib/shadcn/ui/skeleton/index.js';
	import * as Accordion from "$lib/shadcn/ui/accordion";
	import * as Files from '$api/files/endpoints';
	import type { SubItemType } from '$api/files/endpoints';
	import ErrorDisplay from '$lib/components/ErrorDisplay.svelte';
	import type { EndpointError } from '$lib/api/EndpointError';
	
	export let subdirs: string[] = [];
	export let name: string = "";
	export let hidden: boolean = false;
	
	let status = "loading";
	let opened = false;
	let hideChildren = true;
	let items: (SubItemType | undefined)[] = [];
	let error: string | undefined;
	
	function open() {
		hideChildren = !hideChildren;
		
		if (!opened) {
			opened = true;
			fetchSubdirs();
		}
	}
	
	async function fetchSubdirs() {
		console.log("Fetching Subdirectories", subdirs);
		let result = await Files.GET.fetch({ subdirectories: subdirs });
		if (result.err) {
			console.error(result.err);
			error = result.err.message;
		} else {
			items = result.ok.items.flatMap((item) => {
				if (item.type === "directory" && item.name === ".") {
					return item.contents;
				}
			}).sort((a, b) => {
				if (a === undefined || b === undefined) {
					return 0;
				}

				if (a.type === "directory" && b.type === "file") {
					return -1;
				} else if (a.type === "file" && b.type === "directory") {
					return 1;
				} else {
					return a.name.localeCompare(b.name);
				}
			});
			status = "loaded";
		}
	}
</script>

<div class={hidden ? "invisible" : ""}>
<ErrorDisplay {error} />
<Accordion.Root>
	<Accordion.Item value="item-1">
		<Accordion.Trigger on:click={open}>{name}</Accordion.Trigger>
		<Accordion.Content>
			<div class="pl-2 border-accent-foreground border-l-2 min-h-3">
				{#if status === "loading"}
					<Skeleton class="w-full h-[1rem] my-2" />
					<Skeleton class="w-full h-[1rem] my-2" />
					<Skeleton class="w-full h-[1rem] my-2" />
					<Skeleton class="w-full h-[1rem] my-2" />
					<Skeleton class="w-full h-[1rem] my-2" />
				{/if}
				{#if status === "loaded"}
					{#each items as item}
						{#if item?.type === "directory"}
							<svelte:self name={item.name} subdirs={[...subdirs, item.name]} hidden={hideChildren} />
						{:else if item?.type === "file"}
							<div class={`my-2 ${hideChildren? "hidden" : ""} `}>{item.name}</div>
						{/if}
					{/each}
				{/if}
			</div>
		</Accordion.Content>
	</Accordion.Item>
</Accordion.Root>
</div>