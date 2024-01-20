<script>
	import H1 from "$lib/components/H1.svelte";
	import * as Index from "$api/endpoints";
	import {onMount} from "svelte";
	import Dock from "$lib/components/Dock.svelte";
	import AppBounds from "$lib/components/AppBounds.svelte";
	
	let message = "";
	onMount(async () => {
		const res = await Index.GET.fetch({});
		if (res.err) { message = "Welcome, something went wrong... " + res.err.message }
		else { message = res.ok.message }
	})
</script>

<svelte:head>
	<title>Joco</title>
</svelte:head>

<img src="Joco-02.png" alt="Joco">

<AppBounds>
	<Dock>
		<H1>{message}</H1>
	</Dock>
</AppBounds>