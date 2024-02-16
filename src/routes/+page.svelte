<script>
	import H1 from "$lib/components/H1.svelte";
	import {onMount} from "svelte";
	import Dock from "$lib/components/Dock.svelte";
	import AppBounds from "$lib/components/AppBounds.svelte";
	import Meta from '$lib/components/Meta.svelte';
	import InternalFetch from '$lib/api/internalFetch';
	
	let message = "";
	onMount(async () => {
		const res = await InternalFetch("", "GET");
		if (res.type === "err") { message = "Welcome, something went wrong... " + res.error.message }
		else { message = res.data.message }
	})
</script>

<Meta pageTitle="Joco"/>

<img src="/Joco-02.svg" alt="Joco" class="w-full"/>

<AppBounds>
	<Dock>
		<H1>{message}</H1>
	</Dock>
</AppBounds>