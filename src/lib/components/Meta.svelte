<script lang="ts">
	import { env } from '$env/dynamic/public';

	export let pageTitle: string;
	export let pageDescription: string = env.PUBLIC_PROJECT_DESCRIPTION?? "";
	export let pageKeywords: string = "";
</script>

<svelte:head>
	<title>{env.PUBLIC_PROJECT_TITLE?? ""} - {pageTitle}</title>
	<meta name="description" content={pageDescription}/>
	<meta name="keywords" content={env.PUBLIC_PROJECT_BASE_TAGS?? "" + pageKeywords}/>
</svelte:head>