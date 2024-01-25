<script>
	import { onMount } from 'svelte';
	import * as Endpoint from '$api/auth/verify/endpoints';
	import insert_auth from '../insert_auth';

	onMount(async () => {
		const queryParams = new URLSearchParams(window.location.search);
		const token = decodeURIComponent(queryParams.get('token') ?? "");
		const username = decodeURIComponent(queryParams.get('username') ?? "");
		
		const response = await Endpoint.POST.fetch({ token, username });
		if (response.ok) {
			insert_auth({ token, username });
			window.location.href = '/page/profile';
		} else {
			alert("Invalid token, you might want to try signing up again.");
		}
	});
</script>


Verifying...