<script lang="ts">
	import { client_auth } from '$api/auth';
	import * as Permissions from '$api/auth/permissions/endpoints';

	let is_admin = false;
	export let onAuthenticate: () => void = () => {};

	client_auth.subscribe((auth) => {
		if (auth != null) {
			get_perms(auth.username);
		}
		else {
			is_admin = false;
		}
	});

	async function get_perms(username: string) {
		const perms = await Permissions.GET.fetch({ username });
		if (perms.ok) {
			is_admin = perms.ok.permissions.some((perm) => perm.permission === 'admin' || perm.level > 100);
			if (is_admin) {
				onAuthenticate();
			}
		}
	}
</script>

{#if is_admin}
	<slot />
{/if}