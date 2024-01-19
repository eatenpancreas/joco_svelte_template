
import { type UnknownKeysParam, z, type ZodRawShape, type ZodTypeAny } from 'zod';
import { type EndpointError } from '$lib/api/EndpointError';
import { type Result } from '$lib/api/result';

export default class Endpoint
	<IN_T extends ZodRawShape, IN_UnknownKeys extends UnknownKeysParam, IN_Catchall extends ZodTypeAny, IN_Output, IN_Input,
		OUT_T extends ZodRawShape, OUT_UnknownKeys extends UnknownKeysParam, OUT_Catchall extends ZodTypeAny, OUT_Output, OUT_Input> 
{
	async fetch(data: z.infer<typeof this.IN>): Promise<Result<z.infer<typeof this.OUT>, OUT_Input | IN_Input>> {
		try {
			const url = new URL(this.url, window.location.origin);
			const query = new URLSearchParams();
			
			query.set("endpoint_json_data", JSON.stringify(data));
			
			url.search = query.toString();
			const response = await fetch(url.toString(), {
				method: this.method,
			});
			
			if (!response.ok) {
				return { err: { message: `HTTP ${response.status} ${response.statusText} in ${this.method} ${this.url}`}};
			}
			
			const json = await response.json();
			
			if (json.err) { return { err: json.err }; }	

			
			const parsed = this.OUT.safeParse(json.ok);
			if (parsed.success) {
				return { ok: parsed.data };
			} else {
				return { err: { message: "Invalid response", zod_error: parsed.error }};
			}
		} catch (error: any) {
			return { err: { message: error?.message?? "Unknown error"}};
		}
	}
	
	validateDataServer(url: URL): Result<z.infer<typeof this.IN>, OUT_Output | IN_Input> {
		const data = url.searchParams.get("endpoint_json_data");
		if (!data) { return { err: { message: "No data provided"}};}

		const parsed = this.IN.safeParse(JSON.parse(data));

		if (parsed.success) {
			return { ok: parsed.data };
		} else {
			return { err: { message: "Invalid input", zod_error: parsed.error }};
		}
	}

	send(out: z.infer<typeof this.OUT>) {
		return Response.json({ ok: out});
	}
	
	trySend(out: unknown) {
		const parsed = this.OUT.safeParse(out);
		if (parsed.success) {
			return Response.json( { ok: parsed.data });
		} else {
			return Response.json( { err: { message: "Invalid output", zod_error: parsed.error }});
		}
	}

	blindSend(out: unknown) {
		return Response.json({ ok: out });
	}

	error(err: EndpointError<OUT_Input>) {
		return Response.json({ err });
	}
	
	constructor(
		public readonly url: string,
		public readonly method: "GET" | "POST" | "PUT" | "PATCH" | "DELETE",
		private IN: z.ZodObject<IN_T, IN_UnknownKeys, IN_Catchall, IN_Output, IN_Input>,
		private OUT: z.ZodObject<OUT_T, OUT_UnknownKeys, OUT_Catchall, OUT_Output, OUT_Input>,
	) {
	}
}