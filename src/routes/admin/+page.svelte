
<script lang="ts">
	import AppBounds from '$lib/components/AppBounds.svelte';
	import H1 from "$lib/components/H1.svelte";
	import Dock from "$lib/components/Dock.svelte";
	import AdminOnly from '$lib/components/AdminOnly.svelte';
	
	import Meta from '$lib/components/Meta.svelte';
	import { onMount } from 'svelte';
	import TableModals from './TableModals.svelte';
	import Pagination from './Pagination.svelte';
	import Table from './Table.svelte';

	let current_users: {username: string, id: string}[] = [];
	let selected_user: {username: string, id: string} | undefined;

	let warning_open = false;
	let edit_open = false;
	let perms_open = false;
	
	let total = 0;
	let pageNumber = 0;
	
	onMount(() => {
		pageNumber = Number(new URLSearchParams(window.location.search).get("page")) || 0;
	});
	
	async function getUsers() {
		const queryParams = new URLSearchParams(window.location.search);
		queryParams.set("page", pageNumber.toString());
		window.history.replaceState({}, '', `${window.location.pathname}?${queryParams.toString()}`);
		
		// const users = await Users.GET.fetch({ offset: pageNumber * 10});
		// if (users.err) return console.error(users.err);
		// current_users = users.ok.users;
		// total = users.ok.total;
	}
</script>

<Meta pageTitle="Admin"/>

<AdminOnly onAuthenticate={getUsers}>
	<AppBounds>
		<Dock>
			<H1>Admin Panel</H1>
			
			<Table bind:current_users bind:warning_open bind:edit_open bind:perms_open bind:selected_user/>
			<Pagination {getUsers} bind:total bind:pageNumber/>
			<TableModals bind:warning_open bind:edit_open bind:perms_open bind:selected_user/>
		</Dock>
	</AppBounds>
</AdminOnly>