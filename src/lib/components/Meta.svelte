<script lang="ts">
	export let pageTitle: string;
	export let pageDescription: string = import.meta.env.VITE_PROJECT_DESCRIPTION?? "";
	export let pageKeywords: string = "";
</script>

<svelte:head>
	<title>{import.meta.env.VITE_PROJECT_TITLE?? ""} - {pageTitle}</title>
	<meta name="description" content={pageDescription}/>
	<meta name="keywords" content={import.meta.env.VITE_PROJECT_BASE_TAGS?? "" + pageKeywords}/>
</svelte:head>