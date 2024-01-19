<script lang="ts">
	import * as Register from '$api/auth/register/endpoints.js';
	import * as Form from '$lib/shadcn/ui/form';
	import type { SuperValidated } from 'sveltekit-superforms';
	import type { FormOptions } from 'formsnap';
	import { z } from "zod";
	import AppBounds from '$lib/components/AppBounds.svelte';
	import ErrorDisplay from '$lib/components/ErrorDisplay.svelte';
	import { AspectRatio } from '$lib/shadcn/ui/aspect-ratio';
	import { get } from 'svelte/store';
	
	let error: string | undefined;
	
	export let form: SuperValidated<typeof Register.User>;
	const options: FormOptions<typeof Register.User> = {
		validators: Register.User,
		onSubmit: async (data) => {
			// Data here is not in Zod format, we need to convert
			const obj: any = {};
			data.formData.forEach((value, key) => {
				obj[key] = value;
			});

			const parse = Register.User.safeParse(obj);
			if (!parse.success) {
				console.log("Parse errors: ", parse.error);
				error = parse.error.message;
				return;
			}
			
			let result = await Register.POST.fetch(obj);
			console.log("Result:", result);
		},
		onError: (e) => {
			error = get(e.message);
		},
	};
</script>

<svelte:head>
	<title>Profile</title>
</svelte:head>

<ErrorDisplay {error} />

<AppBounds className="h-[70vh] justify-evenly">
	<Form.Root class="w-full" {options} schema={Register.User} {form} let:config>
		<Form.Field {config} name="username">
			<Form.Item>
				<Form.Label />
				<Form.Input name="username" placeholder="Username..." class="rounded-2xl"/>
				<Form.Description />
				<Form.Validation />
			</Form.Item>
		</Form.Field>
		<Form.Field {config} name="password">
			<Form.Item>
				<Form.Label />
				<Form.Input name="password" placeholder="Password..." type="password" class="rounded-2xl"/>
				<Form.Description />
				<Form.Validation />
			</Form.Item>
		</Form.Field>
		<Form.Button class="w-full rounded-2xl">Submit</Form.Button>
	</Form.Root>	
</AppBounds>