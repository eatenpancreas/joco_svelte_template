import * as This from './endpoints';
import { PrismaClient } from '@prisma/client';
import { env } from '$env/dynamic/private';
import pkg from 'bcryptjs';
import jwt from "jsonwebtoken";
import {safeguard, safeguard_async} from "$lib/api/result";
import {asyncSign} from "$lib/gen/asyncify";
const {compare} = pkg;

export async function POST({ url}) {
	const data = This.POST.validateDataServer(url);
	if (data.err) { return This.POST.error(data.err); }
	
	const secret = env.JWT_SECRET;
	if (!secret) return This.POST.errorMsg("Environments are not correct!", "env_not_correct");

	const prisma = new PrismaClient();
	This.POST.dbInit(prisma);
	
	const u = await prisma.user.findFirst({
		where: { username: data.ok.username }
	});
	
	if (u == null) return This.POST.errorMsg("User does not exist!", "user_does_not_exist");

	const pass_is_correct = await compare(data.ok.password, u.password);
	if (!pass_is_correct) return This.POST.errorMsg("Password is incorrect!", "password_incorrect");
	
	const token_verify = safeguard(() => jwt.verify(u.jwt, secret));
	if (token_verify.err) {
		const token = await asyncSign({ username: data.ok.username, password: data.ok.password },
			secret, { expiresIn: '10h' });
		if (token.err) return This.POST.error(token.err);
		
		const result = await safeguard_async(async () => {
			return prisma.user.update({
				where: { username: u.username },
				data: { jwt: token.ok }
			});
		});
		
		if (result.err) { return This.POST.error(result.err); }
		
		return This.POST.send({ message: `User ${result.ok.username} Logged In!`, token: token.ok, username: result.ok.username });
	}

	return This.POST.send({ message: `User ${u.username} Logged In!`, token: u.jwt, username: u.username });
}