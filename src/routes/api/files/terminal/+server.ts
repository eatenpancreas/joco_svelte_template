import * as This from './endpoints';
import ssh_conn from '$lib/server/ssh_conn';
import { env } from '$env/dynamic/private';

export async function GET({ url}) {
	const data = This.GET.validateDataServer(url);
	if (data.err) { return This.GET.error(data.err); }
	
	const conn = await ssh_conn();
	if (conn.err) return This.GET.error(conn.err);
	
	const dir = env.MC_WORKING_DIR;
	if (!dir) return This.GET.error({ message: "Environments are not correct!" });
	
	const result = await conn.ok.exec(data.ok.input, [], { cwd: dir});
	return This.GET.send({ output: result });
}