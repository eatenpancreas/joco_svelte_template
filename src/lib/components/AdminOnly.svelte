<script lang="ts">
	import user from '$lib/http/user';
	import getPermissions from '$api/users/{username}/permissions/get';

	let is_admin = false;

	user.subscribe(u => {
		if (u) {
			get_perms(u.username)
		} else {
			is_admin = false;
		}
	});

	async function get_perms(username: string) {
		const perms = await getPermissions(null, username);
		if (perms.type === 'ok') {
			if (perms.data) {
				is_admin = perms.data.permissions.findIndex(p => p.level >= 8) !== -1;
			} else {
				is_admin = false;
			}
		}
	}
</script>

{#if is_admin}
	<slot />
{/if}