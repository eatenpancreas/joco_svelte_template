import * as This from './endpoints';
import ssh_conn from '$lib/server/ssh_conn';
import { env } from '$env/dynamic/private';

export async function GET({ url}) {
	const data = This.GET.validateDataServer(url);
	if (data.err) { return This.GET.error(data.err); }
	
	const conn = await ssh_conn();
	if (conn.err) return This.GET.error(conn.err);

	// disallow returning out of the directory
	data.ok.subdirectories = data.ok.subdirectories.filter((dir) => dir !== ".." && dir !== ".");
	const dir = env.MC_WORKING_DIR;
	if (!dir) return This.GET.error({ message: "Environments are not correct!" });
	const workingDir = dir + data.ok.subdirectories.join("/");
	const result = await conn.ok.exec(`tree -J -L 1`, [], { cwd: workingDir});

	return This.GET.trySend({ items: JSON.parse(result) });
}