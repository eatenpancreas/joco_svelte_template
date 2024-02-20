import type { ErrorResponse } from '../handshake/ErrorResponse';
import type { OkResponse } from '$lib/handshake/OkResponse';

export type Response<T> = { type: "ok", data?: T, message: string } | { type: "err", error: ErrorResponse };

export async function request_inner<T>(host: string, path: string, searchParams: URLSearchParams, request_init: RequestInit): Promise<Response<T>> {
	const response = await fetch(host + path + "?" + searchParams.toString(), request_init);
	if (!response.ok) {
		try {
			const error = await response.json();
			if (error.message) {
				return { type: "err", error: error.message };
			}
		} catch {
			return {
				type: "err",
				error: {
					message: response.statusText,
					kind: {
						err_kind: "single",
						response: {
							kind: "private_fatal",
							message: "Unknown error!",
							origin: "fetch"
						}
					}
				},
			}
		}
	}

	const json = await response.json() as OkResponse<T>;
	
	if (json.ok.ok_kind == "data") {
		return { type: "ok", message: json.message, data: json.ok.response }
	} else if (json.ok.ok_kind == 'redirected') {
		window.location.href = window.location.host + "/" + json.ok.response.to;
	}
	return { type: "ok", message: json.message }
}