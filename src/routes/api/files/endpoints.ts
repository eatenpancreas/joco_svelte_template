import Endpoint from '$lib/api/Endpoint';
import { z } from "zod";

/** An item contained in a directory */
const SubItem = z.union([z.object({
	type: z.literal("file"),
	name: z.string()
}), z.object({
	type: z.literal("directory"),
	name: z.string()
})]);

export type SubItemType = z.infer<typeof SubItem>;

const Item = z.union([z.object({
	type: z.literal("file"),
	name: z.string()
}), z.object({
	type: z.literal("directory"),
	name: z.string(),
	contents: z.array(SubItem).optional()
}), z.object({
	type: z.literal("report"),
	directories: z.number(),
	files: z.number()
})]);

export const GET = new Endpoint("/api/files", "GET", z.object({
	/** The subdirectories to look in */
	subdirectories: z.array(z.string())
}), z.object({
	/** The items in the directory */
	items: z.array(Item)
}));