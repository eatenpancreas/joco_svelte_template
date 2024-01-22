
<script lang="ts">
	import AppBounds from '$lib/components/AppBounds.svelte';
	import H1 from "$lib/components/H1.svelte";
	import Dock from "$lib/components/Dock.svelte";
	import AdminOnly from '$lib/components/AdminOnly.svelte';
	import * as Table from "$lib/shadcn/ui/table";
	import * as Tooltip from "$lib/shadcn/ui/tooltip";
	import * as Pagination from "$lib/shadcn/ui/pagination";
	
	import * as Users from '$api/crud/user/endpoints';
	import { MagicWand, Pencil1, Trash } from 'radix-icons-svelte';
	import Meta from '$lib/components/Meta.svelte';
	import { onMount } from 'svelte';
	
	let current_users: {username: string, id: string}[] = [];
	let total = 0;
	let pageNumber = 0;
	
	onMount(() => {
		pageNumber = Number(new URLSearchParams(window.location.search).get("page")) || 0;
	});
	async function getUsers() {
		const queryParams = new URLSearchParams(window.location.search);
		queryParams.set("page", pageNumber.toString());
		window.history.replaceState({}, '', `${window.location.pathname}?${queryParams.toString()}`);
		
		const users = await Users.GET.fetch({ offset: pageNumber * 10});
		if (users.err) return console.error(users.err);
		current_users = users.ok.users;
		total = users.ok.total;
	}
</script>

<Meta pageTitle="Admin"/>

<AdminOnly onAuthenticate={getUsers}>
	<AppBounds>
		<Dock>
			<H1>Admin Panel</H1>

			<Table.Root>
				<Table.Header>
					<Table.Row>
						<Table.Head>ID</Table.Head>
						<Table.Head>Username</Table.Head>
						<Table.Head class="text-right">Actions</Table.Head>
					</Table.Row>
				</Table.Header>
				<Table.Body>
					{#each current_users as user}
						<Table.Row>
							<Table.Cell class="font-medium">{user.id}</Table.Cell>
							<Table.Cell>{user.username}</Table.Cell>
							<Table.Cell class="text-end">
								<Tooltip.Root>
									<Tooltip.Trigger><MagicWand/></Tooltip.Trigger>
									<Tooltip.Content class="rounded-xl"><p>Grant Permissions</p></Tooltip.Content>
								</Tooltip.Root>
								<Tooltip.Root>
									<Tooltip.Trigger><Pencil1/></Tooltip.Trigger>
									<Tooltip.Content class="rounded-xl"><p>Edit</p></Tooltip.Content>
								</Tooltip.Root>
								<Tooltip.Root>
									<Tooltip.Trigger><Trash/></Tooltip.Trigger>
									<Tooltip.Content class="rounded-xl"><p>Delete</p></Tooltip.Content>
								</Tooltip.Root>
							</Table.Cell>
						</Table.Row>
					{/each}
				</Table.Body>
			</Table.Root>
			
			
			<Pagination.Root count={total} perPage={10} let:pages let:currentPage page={pageNumber + 1}>
				<Pagination.Content>
					<Pagination.Item>
						<Pagination.PrevButton on:click={() => {
							pageNumber--;
							getUsers();
						}}/>
					</Pagination.Item>
					{#each pages as page (page.key)}
						{#if page.type === "ellipsis"}
							<Pagination.Item>
								<Pagination.Ellipsis />
							</Pagination.Item>
						{:else}
							<Pagination.Item isVisible={currentPage === page.value}>
								<Pagination.Link {page} isActive={currentPage === page.value} on:click={() => {
									pageNumber = page.value;
									getUsers();
								}}>
									{page.value}
								</Pagination.Link>
							</Pagination.Item>
						{/if}
					{/each}
					<Pagination.Item>
						<Pagination.NextButton on:click={() => {
							pageNumber++;
							getUsers();
						}}/>
					</Pagination.Item>
				</Pagination.Content>
			</Pagination.Root>
		</Dock>
	</AppBounds>
</AdminOnly>