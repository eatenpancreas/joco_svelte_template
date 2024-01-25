<script lang="ts">
	import { MagicWand, Pencil1, Trash } from 'radix-icons-svelte';

	import * as Table from "$lib/shadcn/ui/table";
	import * as DropdownMenu from "$lib/shadcn/ui/dropdown-menu";
	import { Checkbox } from '$lib/shadcn/ui/checkbox';
	export let current_users: {username: string, id: string, checked?: boolean}[] = [];
	export let selected_user: {username: string, id: string, checked?: boolean} | undefined;
	
	export let warning_open = false;
	export let edit_open = false;
	export let perms_open = false;
	
	let all_checked = false;
	let main_checked = false;
	
	function onAllCheckedChange(val: boolean | "indeterminate") {
		if (val === "indeterminate") return;
		current_users.forEach(user => user.checked = val);
		
		// this is here because if not, the following will happen:
		// when all selected is FALSE & when user 1 (or group) selected is FALSE
		// but the rest is TRUE
		// and setting all to TRUE
		// user 1 won't be set to TRUE, it will just remain false.
		if (val === true) {
			all_checked = false;
		}
		
		all_checked = val;
		main_checked = val;
	}
	
	function onIndividualCheckedChange(val: boolean) {
		if (val === false) {
			main_checked = false;
		} else {
			main_checked = current_users.every(user => user.checked);
		}
	}
</script>

<Table.Root class="mb-8">
	<Table.Caption>
		<DropdownMenu.Root>
			<DropdownMenu.Trigger>Group Actions ...</DropdownMenu.Trigger>
			<DropdownMenu.Content>
				<DropdownMenu.Group>
					<DropdownMenu.Label>Group Actions</DropdownMenu.Label>
					<DropdownMenu.Separator />
					<DropdownMenu.Item on:click={() => {
									selected_user = user;
									perms_open = true;
								}}><MagicWand class="mr-2"/> Grant Permissions</DropdownMenu.Item>
					<DropdownMenu.Item on:click={() => {
									selected_user = user;
									edit_open = true;
								}}><Pencil1 class="mr-2"/> Edit</DropdownMenu.Item>
					<DropdownMenu.Item on:click={() => {
									selected_user = user;
									warning_open = true;
								}}><Trash class="mr-2"/> Delete</DropdownMenu.Item>
				</DropdownMenu.Group>
			</DropdownMenu.Content>
		</DropdownMenu.Root>
	</Table.Caption>
	
	
	<Table.Header>
		<Table.Row>
			<Table.Head><Checkbox class="rounded-xl" onCheckedChange={onAllCheckedChange} checked={main_checked}/></Table.Head>
			<Table.Head>ID</Table.Head>
			<Table.Head>Username</Table.Head>
			<Table.Head class="text-right">Actions</Table.Head>
		</Table.Row>
	</Table.Header>
	<Table.Body>
		{#each current_users as user}
			<Table.Row>
				<Table.Cell class="font-medium"><Checkbox class="rounded-xl" checked={all_checked} onCheckedChange={val => {
					if (val === "indeterminate") return;
					user.checked = val;
					onIndividualCheckedChange(val);
				}}/></Table.Cell>
				<Table.Cell class="font-medium">{user.id}</Table.Cell>
				<Table.Cell>{user.username}</Table.Cell>
				<Table.Cell class="text-end">
					<DropdownMenu.Root>
						<DropdownMenu.Trigger>...</DropdownMenu.Trigger>
						<DropdownMenu.Content>
							<DropdownMenu.Group>
								<DropdownMenu.Label>Actions</DropdownMenu.Label>
								<DropdownMenu.Separator />
								<DropdownMenu.Item on:click={() => {
									selected_user = user;
									perms_open = true;
								}}><MagicWand class="mr-2"/> Grant Permissions</DropdownMenu.Item>
								<DropdownMenu.Item on:click={() => {
									selected_user = user;
									edit_open = true;
								}}><Pencil1 class="mr-2"/> Edit</DropdownMenu.Item>
								<DropdownMenu.Item on:click={() => {
									selected_user = user;
									warning_open = true;
								}}><Trash class="mr-2"/> Delete</DropdownMenu.Item>
							</DropdownMenu.Group>
						</DropdownMenu.Content>
					</DropdownMenu.Root>
				</Table.Cell>
			</Table.Row>
		{/each}
	</Table.Body>
</Table.Root>