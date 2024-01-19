
import Endpoint from '$lib/api/Endpoint';
import { z } from "zod";

export const User = z.object({
	username: z.string().min(2).max(255),
	password: z.string().min(8).max(60),
});

const Output = z.object({
	message: z.string(),
	token: z.string(),
});

export const POST = new Endpoint("/api/auth/register", 'POST', User, Output);