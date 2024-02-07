<script lang="ts">
    import * as Login from "$api/auth/login/endpoints.js";
    import * as Form from '$lib/shadcn/ui/form';
    import type {SuperValidated} from "sveltekit-superforms";
    import ErrorDisplay from "$lib/components/ErrorDisplay.svelte";
    import H1 from "$lib/components/H1.svelte";
    
    export let form: SuperValidated<typeof Login.User>;
    export let onAuthenticate: (res: {message: string, token: string, username: string}) => void;

    let title = "Log in";
    let error: string | undefined;
    
    const options = Login.POST.formOptions((callResponse) => {
        if (callResponse.err) error = callResponse.err.message;
        else { onAuthenticate(callResponse.ok); }
    }, Login.User)
</script>

<div class="relative">
    <ErrorDisplay {error} />
</div>

<H1>{title}</H1>

<Form.Root id="login-form" class="w-full" {options} schema={Login.User} {form} let:config>
    <Form.Field {config} name="username">
        <Form.Item>
            <Form.Label>Username</Form.Label>
            <Form.Input name="username" placeholder="Username..." class="rounded-xl"/>
            <Form.Description />
            <Form.Validation />
        </Form.Item>
    </Form.Field>
    <Form.Field {config} name="password">
        <Form.Item>
            <Form.Label>Password</Form.Label>
            <Form.Input name="password" placeholder="Password..." type="password" class="rounded-xl"/>
            <Form.Description />
            <Form.Validation />
        </Form.Item>
    </Form.Field>
    <Form.Button class="w-full rounded-xl">Submit</Form.Button>
</Form.Root>