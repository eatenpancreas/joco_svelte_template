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
		users: z.array(z.intersection(Username, z.object({ 
			permissions: z.array(z.object({
				permission: z.object({
					id: z.string(),
					permission: z.string(),
					level: z.number(),
					created_at: z.date()
				}),
				user_id: z.string(),
				permission_id: z.string()
			})),
			id: z.string(),
			email: z.string(),
			created_at: z.date(),
			last_login: z.date(),
			is_verified: z.boolean(),
		}))),
		total: z.number(),
		is_end: z.boolean(),
		message: z.string(),
	})
);