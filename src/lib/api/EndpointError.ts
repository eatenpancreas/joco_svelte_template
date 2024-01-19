import { ZodError } from 'zod';

export type EndpointError<T> = {
	message: string;
	zod_error?: ZodError<T>;
}