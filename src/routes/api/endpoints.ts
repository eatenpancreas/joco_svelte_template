
import Endpoint from '$lib/api/Endpoint';
import { z } from "zod";

export const Message = z.object({
	message: z.string(),
});

export const GET = new Endpoint("/api", "GET", z.object({}), Message);