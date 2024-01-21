
import {errMsg, type Result} from '$lib/api/result';

process.on('unhandledRejection', (reason) => {
	const err = reason as Error;
	console.error('Unhandled Rejection:', err.message);
});

export default function<T, E> (max_execution_time: number, timeout_function: () => Promise<T>): Promise<Result<T, E>> {
	return new Promise(resolve => {
		const timeout = setTimeout(() => {
			resolve( errMsg("Timed out at" + max_execution_time + " MS.", "timeout"));
		}, max_execution_time);

		try {
			timeout_function().then((v) => {
				clearTimeout(timeout);
				resolve({ ok: v });
			});
		} catch {
			resolve( errMsg("Timed out at" + max_execution_time + " MS.", "timeout"));
		}
	});
}

