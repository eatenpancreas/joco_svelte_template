import { z } from 'zod';

export const ListOptions = z.object({
	offset: z.number().int().min(0).optional(),
	limit: z.number().int().min(0).max(100).optional(),
});