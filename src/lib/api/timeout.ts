
import { type Result } from '$lib/api/result';

process.on('unhandledRejection', (reason, promise) => {
	const err = reason as Error;
	console.error('Unhandled Rejection:', err.message);
});

export default function<T, E> (max_execution_time: number, timeout_function: () => Promise<T>): Promise<Result<T, E>> {
	return new Promise(resolve => {
		const timeout = setTimeout(() => {
			resolve({ err: { message: "Timed out at " + max_execution_time + " MS." } });
		}, max_execution_time);

		try {
			timeout_function().then((v) => {
				clearTimeout(timeout);
				resolve({ ok: v });
			});
		} catch {
			resolve({ err: { message: "There was a connection error between the servers!" }});
		}
	});
}

