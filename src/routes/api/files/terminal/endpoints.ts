import Endpoint from '$lib/api/Endpoint';
import { z } from "zod";
	
export const GET = new Endpoint("/api/files/terminal", "GET", z.object({
	/** The user's given input */
	directory: z.string(),
	input: z.string()
}), z.object({
	/** The output of the command */
	output: z.string()
}));