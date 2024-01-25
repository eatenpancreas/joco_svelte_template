
import Endpoint from '$lib/api/Endpoint';
import { z } from 'zod';
import {ClientAuth} from "$api/auth";

export const User = z.object({
	username: z.string().min(2).max(255),
	password: z.string().min(8).max(60),
	confirm_password: z.string().min(8).max(60),
	email: z.string().email().max(255)
});

const RegisterClientAuth = z.object({
	message: z.string(),
	username: z.string().optional(),
	token: z.string().optional(),
});

export const POST = new Endpoint("/api/auth/register", 'POST', User, RegisterClientAuth);