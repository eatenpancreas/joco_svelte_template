import * as Endpoint from './endpoints';
import { env } from '$env/dynamic/private';
import pkg from 'bcryptjs';
import jwt from "jsonwebtoken";
import {safeguard, safeguard_async} from "$lib/api/result";
import {asyncSign} from "$lib/gen/asyncify";
import { prisma } from '$lib/gen/db';
const {compare} = pkg;

export async function POST({ url}) {
	const This = Endpoint.POST;
	const data = This.validateDataServer(url);
	if (data.err) { return This.error(data.err); }
	
	const secret = env.JWT_SECRET;
	if (!secret) return This.errorMsg("Environments are not correct!", "env_not_correct");
	
	const u = await prisma.user.findFirst({
		where: { username: data.ok.username }
	});
	
	if (u == null) return This.errorMsg("User does not exist!", "user_does_not_exist");

	const pass_is_correct = await compare(data.ok.password, u.password);
	if (!pass_is_correct) return This.errorMsg("Password is incorrect!", "password_incorrect");
	
	const token_verify = safeguard(() => jwt.verify(u.jwt, secret));
	if (token_verify.err) {
		const token = await asyncSign({ username: data.ok.username, password: data.ok.password },
			secret, { expiresIn: '10h' });
		if (token.err) return This.error(token.err);
		
		const result = await safeguard_async(async () => {
			return prisma.user.update({
				where: { username: u.username },
				data: { jwt: token.ok }
			});
		});
		
		if (result.err) { return This.error(result.err); }
		
		return This.send({ message: `User ${result.ok.username} Logged In!`, token: token.ok, username: result.ok.username });
	}

	return This.send({ message: `User ${u.username} Logged In!`, token: u.jwt, username: u.username });
}