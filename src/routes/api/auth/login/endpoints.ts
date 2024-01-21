
import Endpoint from '$lib/api/Endpoint';
import { z } from "zod";
import {ClientAuth} from "$api/auth";

export const User = z.object({
	username: z.string().min(2).max(255),
	password: z.string().min(8).max(60),
});

export const POST = new Endpoint("/api/auth/login", 'POST', User, ClientAuth);