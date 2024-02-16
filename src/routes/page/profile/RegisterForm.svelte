<script lang="ts">
    import * as Form from '$lib/shadcn/ui/form';
    import ErrorDisplay from "$lib/components/ErrorDisplay.svelte";
    import H1 from "$lib/components/H1.svelte";
		import { z } from 'zod';
    
    const RegisterUser = z.object({
				username: z.string().min(3).max(20),
				password: z.string().min(8).max(100),
				confirm_password: z.string().min(8).max(100),
				email: z.string().email(),
		}).superRefine(({ password, confirm_password }, ctx) => {
        if (password !== confirm_password) {
            ctx.addIssue({
                code: "invalid_arguments",
                argumentsError: ZodError.create([
                    {
                        code: "custom",
                        path: ["confirm_password"],
                        fatal: true,
                        message: "The passwords are not the same"
                    },
                ]),
                message: "The passwords are not the same"
            });
        }
    });
    //
    // export let form: SuperValidated<typeof RegisterUser>;
    // export let supertitle: string | undefined;
    // export let onAuthenticate: (res: {message: string, token?: string, username?: string}) => void;
    //
    // let title = "Register";
    // let error: string | undefined;
    //
    // const options = Register.POST.formOptions((callResponse) => {
    //     if (callResponse.err) error = callResponse.err.message;
    //     else { onAuthenticate(callResponse.ok); }
    // }, RegisterUser)
</script>

<!--<div class="relative">-->
<!--    <ErrorDisplay {error} />-->
<!--</div>-->

<!--<H1>{supertitle?? title}</H1>-->

<!--<Form.Root id="register-form" class="w-full" {options} schema={RegisterUser} {form} let:config>-->
<!--    <Form.Field {config} name="username">-->
<!--        <Form.Item>-->
<!--            <Form.Label>Username</Form.Label>-->
<!--            <Form.Input name="username" placeholder="Username..." class="rounded-xl"/>-->
<!--            <Form.Description />-->
<!--            <Form.Validation />-->
<!--        </Form.Item>-->
<!--    </Form.Field>-->
<!--    <Form.Field {config} name="password">-->
<!--        <Form.Item>-->
<!--            <Form.Label>Password</Form.Label>-->
<!--            <Form.Input name="password" placeholder="Password..." type="password" class="rounded-xl"/>-->
<!--            <Form.Description />-->
<!--            <Form.Validation />-->
<!--        </Form.Item>-->
<!--    </Form.Field>-->
<!--    <Form.Field {config} name="confirm_password">-->
<!--        <Form.Item>-->
<!--            <Form.Label>Confirm Password</Form.Label>-->
<!--            <Form.Input name="confirm_password" placeholder="Confirm Password..." type="password" class="rounded-xl"/>-->
<!--            <Form.Description />-->
<!--            <Form.Validation />-->
<!--        </Form.Item>-->
<!--    </Form.Field>-->
<!--    <Form.Field {config} name="email">-->
<!--        <Form.Item>-->
<!--            <Form.Label>E-mail</Form.Label>-->
<!--            <Form.Input name="email" placeholder="E-mail..." type="email" class="rounded-xl"/>-->
<!--            <Form.Description />-->
<!--            <Form.Validation />-->
<!--        </Form.Item>-->
<!--    </Form.Field>-->
<!--    <Form.Button class="w-full rounded-xl">Submit</Form.Button>-->
<!--</Form.Root>-->