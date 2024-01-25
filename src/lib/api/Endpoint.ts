/* eslint-disable @typescript-eslint/no-explicit-any */

import { type UnknownKeysParam, z, type ZodRawShape, ZodType, type ZodTypeAny } from 'zod';
import {type EndpointError} from '$lib/api/EndpointError';
import {errMsg, handle_err, type Result} from '$lib/api/result';
import type { FormOptions } from 'formsnap';
import {get} from "svelte/store";
import {client_auth} from "$api/auth";


/**
 * A wrapper around an endpoint, used by the client to communicate with the server, and vice versa
 * @constructor
 * @param url The url of the endpoint. This is important, and breaks the endpoint if it isn't correct
 * @param method The method of the endpoint. This will send the request to the server with the correct method. If this is
 * not in line with +server.ts, the endpoint will break.
 * @param IN Zod object - The input type of the endpoint. This is used to validate the input data, and to generate forms. 
 * @param OUT Zod object - The output type of the endpoint. This is used to validate the output data
 * 
 * @example ```ts
 * import Endpoint from '$lib/api/Endpoint';
 * import { z } from "zod";
 * import {ClientAuth} from "$api/auth/index";
 * 
 * export const User = z.object({
 * 	username: z.string().min(2).max(255),
 * 	password: z.string().min(8).max(60),
 * });
 * 
 * export const POST = new Endpoint("/api/auth/login", 'POST', User, ClientAuth);
 * ```
 */
export default class Endpoint
	<IN_T extends ZodRawShape, IN_UnknownKeys extends UnknownKeysParam, IN_Catchall extends ZodTypeAny, IN_Output, IN_Input,
		OUT_T extends ZodRawShape, OUT_UnknownKeys extends UnknownKeysParam, OUT_Catchall extends ZodTypeAny, OUT_Output, OUT_Input> 
{
	/**
	 * Fetches the endpoint from the client, returning the output type
	 * @param data The data to send to the endpoint
	 * @returns The result of the endpoint
	 */
	async fetch(data: z.infer<typeof this.IN>): Promise<Result<z.infer<typeof this.OUT>, OUT_Input | IN_Input>> {
		try {
			const auth = get(client_auth);
			const url = new URL(this.url, window.location.origin);
			
			const query = new URLSearchParams();
			query.set("endpoint_json_data", encodeURIComponent(JSON.stringify(data)));
			
			url.search = query.toString();

			const headers = new Headers();
			if (auth != null) {
				headers.set("Authorization", `Bearer ${auth.jwt}`);
			}
			headers.set("Content-Type", "application/json");
			
			const response = await fetch(url.toString(), {
				method: this.method, headers
			});
			
			if (!response.ok) 
				return errMsg(`HTTP ${response.status} ${response.statusText} in ${this.method} ${this.url}`, "HTTP_" + response.status);
			
			const json = await response.json();
			if (json.err) {
				if (json.err.message === "jwt expired") {
					client_auth.set(null);
					if (window.location.pathname !== "/page/profile") {
						const from = window.location.pathname + window.location.search;
						window.location.href = "/page/profile?from=" + encodeURIComponent(from);
					}
				}
				return { err: json.err }; 
			}	
			
			const parsed = this.OUT.safeParse(json.ok);
			if (parsed.success) return { ok: parsed.data };
			else return errMsg("Invalid response", "invalid_response_in_fetch");
		} catch (error: any) {
			return handle_err(error);
		}
	}
	
	/**
	 * Validates the data sent from the client
	 * @param url The url data
	 * @returns The result of the validation
	 */
	validateDataServer(url: URL): Result<z.infer<typeof this.IN>, OUT_Output | IN_Input> {
		const data = url.searchParams.get("endpoint_json_data");
		if (!data) return errMsg("No data", "no_data_in_validateDataServer");

		const parsed = this.IN.safeParse(JSON.parse(decodeURIComponent(data)));

		if (parsed.success) return { ok: parsed.data };
		else return { err: { message: "Invalid input", zod_error: parsed.error, id: "invalid_input_in_validateDataServer" } };
	}

	/**
	 * Sends an OK response to the client
	 * 
	 * This is the default way to send data to the client, with validated data
	 * @param out The data to send to the client
	 * 
	 */
	send(out: z.infer<typeof this.OUT>) {
		const parsed = this.OUT.safeParse(out);
		if (parsed.success) return Response.json( { ok: parsed.data });
		else return Response.json( { err: { message: "Invalid output", zod_error: parsed.error, id: "invalid_output_in_send" }});
	}
	
	/**
	 * Sends an OK response to the client, but first validates the data
	 * 
	 * Useful when trying to send some unknown data, but you want to make sure it arrives in a specific format
	 * @param out Unvalidated data to send to the client
	 */
	trySend(out: unknown) {
		const parsed = this.OUT.safeParse(out);
		if (parsed.success) return Response.json( { ok: parsed.data });
		else return Response.json( { err: { message: "Invalid output", zod_error: parsed.error, id: "invalid_output_in_trySend" }});
	}

	/**
	 * Sends an OK response to the client, but does not validate the data
	 * 
	 * Useful when debugging, its use is otherwise discouraged
	 * @param out Unvalidated data to send to the client
	 */
	blindSend(out: unknown) {
		return Response.json({ ok: out });
	}

	/**
	 * Sends an error response to the client
	 * @param err The error to send to the client
	 */
	error(err: EndpointError<OUT_Input>) {
		return Response.json({ err });
	}
	
	/**
	 * Sends an error response to the client, with a simple message
	 * 
	 */
	errorMsg(message: string, id: string) {
		return Response.json(errMsg(message, id));
	}
	
	/**
	 * Generates a form options object for use with formsnap
	 * @param onResult The function to call when the form is submitted
	 * @param schema The schema to validate
	 */
	// @ts-expect-error/in_type_is_valid
	formOptions<T extends ZodType>(onResult: (result: Result<z.infer<typeof this.OUT>, OUT_Input | IN_Input>) => void, schema: T): FormOptions<T> {
		
		return {
			warnings: {
				noValidationAndConstraints: false,
			},
			id: "form-to-endpoint-" + this.method + "-at-" + this.url,
			// @ts-expect-error Validators are true
			validators: schema,
			dataType: "json",	
			
			onSubmit: (data) => {
				// Data here is not in Zod format, we need to convert
				const obj: any = {};
				data.formData.forEach((value, key) => {
					obj[key] = value;
				});

				const parse = schema.safeParse(obj);
				if (!parse.success) {
					return { err: parse.error };
				}

				this.fetch(obj).then(onResult);
			},
			onError: (e) => {
				const message = get(e.message);
				if (message !== undefined && typeof message === "string") {
					onResult(errMsg(message, "form_error"));
				}
			},
		};
	}
	
	constructor(
		public readonly url: string,
		public readonly method: "GET" | "POST" | "PUT" | "PATCH" | "DELETE",
		private IN: z.ZodObject<IN_T, IN_UnknownKeys, IN_Catchall, IN_Output, IN_Input>,
		private OUT: z.ZodObject<OUT_T, OUT_UnknownKeys, OUT_Catchall, OUT_Output, OUT_Input>,
	) {
	}
}