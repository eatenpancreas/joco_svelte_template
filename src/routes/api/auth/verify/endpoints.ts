import Endpoint from '$lib/api/Endpoint';
import { ClientAuth } from '$api/auth';
import { z } from 'zod';

export const POST = new Endpoint("/api/auth/verify", 'POST', z.object({
	token: z.string(),
	username: z.string()
}), ClientAuth);