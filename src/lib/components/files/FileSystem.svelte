

<script lang="ts">
	import Directory from '$lib/components/files/Directory.svelte';
	import { AspectRatio } from '$lib/shadcn/ui/aspect-ratio';
</script>

<AspectRatio ratio={16 / 9} class="w-full">
	<div class="bg-accent rounded-2xl p-4 h-full w-full overflow-scroll">
		<Directory name="MC Server"/>
	</div>
</AspectRatio>