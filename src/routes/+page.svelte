<script>
	import H1 from "$lib/components/H1.svelte";
	import * as Index from "$api/endpoints";
	import {onMount} from "svelte";
	import Dock from "$lib/components/Dock.svelte";
	import AppBounds from "$lib/components/AppBounds.svelte";
	import Image from "svimg/Image.svelte";
	
	let message = "";
	onMount(async () => {
		const res = await Index.GET.fetch({});
		if (res.err) { message = "Welcome, something went wrong... " + res.err.message }
		else { message = res.ok.message }
	})
</script>

<svelte:head>
	<title>Svelte Template - Joco</title>
</svelte:head>

<Image src="Joco-02.png" alt="Joco" class="w-full"/>

<AppBounds>
	<Dock>
		<H1>{message}</H1>
	</Dock>
</AppBounds>