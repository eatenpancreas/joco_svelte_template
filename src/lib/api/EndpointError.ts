import { ZodError } from 'zod';

export type EndpointError<T> = {
	message: string;
	id: string;
	location?: string;
	children?: EndpointError<T>[];
	
	zod_error?: ZodError<T>;
}
