<script lang="ts">
    import * as Form from '$lib/shadcn/ui/form';
    import ErrorDisplay from "$lib/components/ErrorDisplay.svelte";
    import H1 from "$lib/components/H1.svelte";
		import type { SuperValidated } from 'formsnap';
    import { registerUser, type RegisterUser } from './auth_commons';

    export let form: SuperValidated<RegisterUser>;
</script>

<H1>Register</H1>

<Form.Root id="register-form" class="w-full" action="/profile/register" schema={registerUser} {form} let:config>
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
    <Form.Field {config} name="confirm_password">
        <Form.Item>
            <Form.Label>Confirm Password</Form.Label>
            <Form.Input name="confirm_password" placeholder="Confirm Password..." type="password" class="rounded-xl"/>
            <Form.Description />
            <Form.Validation />
        </Form.Item>
    </Form.Field>
    <Form.Field {config} name="email">
        <Form.Item>
            <Form.Label>E-mail</Form.Label>
            <Form.Input name="email" placeholder="E-mail..." type="email" class="rounded-xl"/>
            <Form.Description />
            <Form.Validation />
        </Form.Item>
    </Form.Field>
    <Form.Button class="w-full rounded-xl">Submit</Form.Button>
</Form.Root>