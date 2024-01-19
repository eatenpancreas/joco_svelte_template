
<script lang="ts">
	import FileSystem from '$lib/components/files/FileSystem.svelte';
	import AppBounds from '$lib/components/AppBounds.svelte';
	import SshConnectionVerifier from '$lib/components/files/SshConnectionVerifier.svelte';
	import Terminal from '$lib/components/terminal/Terminal.svelte';
	import * as Tabs from "$lib/shadcn/ui/tabs";
</script>

<svelte:head>
	<title>Admin</title>
</svelte:head>

<SshConnectionVerifier />

<AppBounds className="gap-2">
	<Tabs.Root value="files" class="w-full">
		<Tabs.List class="rounded-2xl">
			<Tabs.Trigger class="rounded-2xl" value="files">MC Server files</Tabs.Trigger>
			<Tabs.Trigger class="rounded-2xl" value="screen">MC Server screen</Tabs.Trigger>
			<Tabs.Trigger class="rounded-2xl" value="services">SystemCTL Services</Tabs.Trigger>
		</Tabs.List>
	</Tabs.Root>
	<Tabs.Root value="files" class="w-full">
		<Tabs.List class="rounded-2xl w-full">
			<Tabs.Trigger class="rounded-2xl" value="files">File System</Tabs.Trigger>
			<Tabs.Trigger class="rounded-2xl" value="term">Terminal</Tabs.Trigger>
			<Tabs.Trigger class="rounded-2xl" value="editor">Editor</Tabs.Trigger>
			<Tabs.Trigger class="rounded-2xl" value="perf">Performance</Tabs.Trigger>
		</Tabs.List>
		<Tabs.Content value="term">
			<Terminal />
		</Tabs.Content>
		<Tabs.Content value="files">
			<FileSystem />
		</Tabs.Content>
	</Tabs.Root>
</AppBounds>
