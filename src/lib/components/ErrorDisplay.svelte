<script lang="ts">
	import { ExclamationTriangle } from 'radix-icons-svelte';
	import * as Alert from "$lib/shadcn/ui/alert";
	export let error: string | undefined;
	export let title = 'Error';
</script>

{#if error}
	<div class="absolute top-0 pt-8">
		<Alert.Root variant="destructive" class="bg-primary-foreground rounded-2xl">
			<ExclamationTriangle class="h-4 w-4" />
			<Alert.Title>{title}</Alert.Title>
			<Alert.Description>{error}</Alert.Description>
		</Alert.Root>
	</div>
{/if}