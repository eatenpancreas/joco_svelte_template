
import { z, ZodError } from 'zod';

export const registerUser = z.object({
	username: z.string().min(2).max(64),
	password: z.string().min(6).max(64),
	confirm_password: z.string().min(6).max(64),
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

export type RegisterUser = typeof registerUser;