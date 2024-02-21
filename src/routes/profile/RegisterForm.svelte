<script lang="ts">
    import * as Form from '$lib/shadcn/ui/form';
    import ErrorDisplay from "$lib/components/ErrorDisplay.svelte";
    import H1 from "$lib/components/H1.svelte";
		import type { SuperValidated } from 'formsnap';
    import { registerUser, type RegisterUser } from './auth_commons';
    import { form_options } from '$lib/http/form';
    import post from '$api/auth/register/post';
    import { z } from "zod";
    import user from '$lib/http/user';
    import type { ErrorResponse } from '$lib/handshake/ErrorResponse';

    export let form: SuperValidated<RegisterUser>;
    let error: ErrorResponse | undefined;
    
    const options = form_options(registerUser, (async (obj) => {
        let parsed = registerUser.parse(obj);
        
        let result = await post(parsed);
        if (result.type == "ok") {
            user.set(result.data);
        } else {
            error = result.error;
        }
    }))
</script>

<ErrorDisplay {error} />


<H1>Register</H1>

<Form.Root id="register-form" class="w-full" {options} action="/profile/register" schema={registerUser} {form} let:config>
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