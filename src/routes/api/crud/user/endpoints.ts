import Endpoint from '$lib/api/Endpoint';
import { Username } from '$api/auth';
import { z } from 'zod';
import { ListOptions } from '$api/crud';

/**
 * Deletes a user
 */
export const DELETE = new Endpoint(
	"/api/crud/user", 'DELETE',
	Username, z.object({
		user: Username,
		message: z.string(),
	})
);

/**
 * Lists users
 */
export const GET = new Endpoint(
	"/api/crud/user", 'GET',
	ListOptions, z.object({
		users: z.array(z.intersection(Username, z.object({ id: z.string() }))),
		is_end: z.boolean(),
		message: z.string(),
	})
);