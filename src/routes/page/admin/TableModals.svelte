<script lang="ts">
	import * as AlertDialog from "$lib/shadcn/ui/alert-dialog";
	import * as Sheet from "$lib/shadcn/ui/sheet";

	export let warning_open = false;
	export let edit_open = false;
	export let perms_open = false;
	export let selected_user: {username: string, id: string} | undefined;
</script>

<!-- Delete User Warning -->
<AlertDialog.Root bind:open={warning_open}>
	<AlertDialog.Content>
		<AlertDialog.Header>
			<AlertDialog.Title>Are you absolutely sure?</AlertDialog.Title>
			<AlertDialog.Description>
				This action cannot be undone. This will permanently delete {selected_user?.username ?? "undefined"}
				and remove their data from our servers.
			</AlertDialog.Description>
		</AlertDialog.Header>
		<AlertDialog.Footer>
			<AlertDialog.Cancel>Cancel</AlertDialog.Cancel>
			<AlertDialog.Action>Continue</AlertDialog.Action>
		</AlertDialog.Footer>
	</AlertDialog.Content>
</AlertDialog.Root>

<!-- Edit sheet -->
<Sheet.Root bind:open={edit_open}>
	<Sheet.Content>
		<Sheet.Header>
			<Sheet.Title>Edit {selected_user?.username ?? "undefined"}</Sheet.Title>
			<Sheet.Description>
				This action cannot be undone. This will permanently delete your account
				and remove your data from our servers.
			</Sheet.Description>
		</Sheet.Header>
	</Sheet.Content>
</Sheet.Root>

<!-- Grant permissions sheet -->
<Sheet.Root bind:open={perms_open}>
	<Sheet.Content>
		<Sheet.Header>
			<Sheet.Title>Give permissions to {selected_user?.username ?? "undefined"}</Sheet.Title>
			<Sheet.Description>
				This action cannot be undone. This will permanently delete your account
				and remove your data from our servers.
			</Sheet.Description>
		</Sheet.Header>
	</Sheet.Content>
</Sheet.Root>