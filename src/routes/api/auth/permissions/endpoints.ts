
import Endpoint from '$lib/api/Endpoint';
import { z } from "zod";
import { Username } from '$api/auth';


export const Permission= z.object({
	permission: z.string(),
	level: z.number(),
});

export const GET = new Endpoint(
	"/api/auth/permissions", 'GET',
	Username, z.object({
		username: z.string(),
		permissions: z.array(Permission),
		password: z.string().optional(),
	})
);