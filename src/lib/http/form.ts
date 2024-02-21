import type { SuperFormOptions, ZodValidation } from 'formsnap';
import { type AnyZodObject, type SafeParseSuccess, z } from 'zod';

export function form_options<T extends ZodValidation<AnyZodObject>, M>(schema: T, on_submit = (z: any) => {}): SuperFormOptions<T, M> {
	return {
		warnings: {
			noValidationAndConstraints: false,
		},
		id: "form-to-endpoint-",
		// @ts-expect-error Validators are true
		validators: schema,
		dataType: "json",

		onSubmit: (data) => {
			// Data here is not in Zod format, we need to convert
			const obj: any = {};
			data.formData.forEach((value, key) => {
				obj[key] = value;
			});
			
			on_submit(obj);
		},
		onError: (e) => {
		},
	}
}