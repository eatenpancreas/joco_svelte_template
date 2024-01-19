
<script lang="ts">
	import { onMount } from 'svelte';
	import * as Index from '$api/endpoints';
	import * as Alert from "$lib/shadcn/ui/alert";
	import { ExclamationTriangle } from 'radix-icons-svelte';
	import ErrorDisplay from '$lib/components/ErrorDisplay.svelte';

	let error: string | undefined;
	onMount(async () => {
		let result = await Index.GET.fetch({});
		if (result.err) {
			error = result.err.message;
		}
	});
</script>

<ErrorDisplay {error} />
