<script lang="ts">
    import * as Register from "$api/auth/register/endpoints.js";
    import * as Form from '$lib/shadcn/ui/form';
    import type {SuperValidated} from "sveltekit-superforms";
    import ErrorDisplay from "$lib/components/ErrorDisplay.svelte";
    import H1 from "$lib/components/H1.svelte";
    
    export let form: SuperValidated<typeof Register.User>;
    export let onAuthenticate: (res: {message: string, token: string, username: string}) => void;

    let title = "Register";
    let error: string | undefined;
    
    const options = Register.POST.formOptions((callResponse) => {
        if (callResponse.err) error = callResponse.err.message;
        else { onAuthenticate(callResponse.ok); }
    })
</script>

<div class="relative">
    <ErrorDisplay {error} />
</div>

<H1>{title}</H1>

<Form.Root id="register-form" class="w-full" {options} schema={Register.User} {form} let:config>
    <Form.Field {config} name="username">
        <Form.Item>
            <Form.Label>Username</Form.Label>
            <Form.Input name="username" placeholder="Username..." class="rounded-2xl"/>
            <Form.Description />
            <Form.Validation />
        </Form.Item>
    </Form.Field>
    <Form.Field {config} name="password">
        <Form.Item>
            <Form.Label>Password</Form.Label>
            <Form.Input name="password" placeholder="Password..." type="password" class="rounded-2xl"/>
            <Form.Description />
            <Form.Validation />
        </Form.Item>
    </Form.Field>
    <Form.Button class="w-full rounded-2xl">Submit</Form.Button>
</Form.Root>