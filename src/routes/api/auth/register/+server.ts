import * as Endpoint from './endpoints';
import { env } from '$env/dynamic/private';
import {asyncSign} from "$lib/gen/asyncify";
import {safeguard_async} from "$lib/api/result";
import pkg from 'bcryptjs';
import { prisma } from '$lib/gen/db';
import transporter from '$lib/api/transporter';
const {genSalt, hash} = pkg;

export async function POST({ url}) {
	const This = Endpoint.POST;
	
	const data = This.validateDataServer(url);
	if (data.err) { return This.error(data.err); }
	
	const secret = env.JWT_SECRET;
	if (!secret) return This.errorMsg("Environments are not correct!", "env_not_correct");
	
	const token = await asyncSign({ username: data.ok.username, password: data.ok.password }, 
		secret, { expiresIn: '10h' });
	
	if (token.err) return This.error(token.err);

	const u = await prisma.user.findFirst({
		where: { username: data.ok.username }
	});
	if (u) return This.errorMsg("Username already exists!", "username_exists");
	
	const e = await prisma.user.findFirst({
		where: { email: data.ok.email }
	});
	if (e) return This.errorMsg("Email already exists!", "email_exists");
	
	const result = await safeguard_async(async () => {
		const salt = await genSalt();
		const hashed = await hash(data.ok.password, salt);
		
		return prisma.user.create({
			data: {
				username: data.ok.username,
				password: hashed,
				jwt: token.ok,
				email: data.ok.email
			}
		});
	});
	
	if (result.err) { return This.error(result.err); }
	
	if (env.EMAIL_AUTH_ENABLED == "true") {
		const random_number = Math.floor(Math.random() * 1000000).toString();
		const register_token = await asyncSign({ 
			random_number
		}, secret, { expiresIn: '10m' });
		
		if (register_token.err) return This.error(register_token.err);
		
		
		const unverif_create = await safeguard_async(() => prisma.unverifiedUser.create({
			data: {
				auth_jwt: register_token.ok,
				user_id: result.ok.id
			}
		}));
		
		if (unverif_create.err) return This.error(unverif_create.err);
		
		// Sending verification email

		const url = new URL("/page/profile/verify", env.VITE_PROJECT_URL);
		const query = new URLSearchParams();
		query.set("username", data.ok.username);
		query.set("token", register_token.ok);
		url.search = query.toString();
		
		await transporter.sendMail({
			from: env.EMAIL_USER,
			to: data.ok.email,
			subject: "Verification e-mail from " + env.VITE_PROJECT_TITLE,
			text: `Verify your token!
				Was this not you? Then please ignore this message, or let us know by sending a reply.
				Please click or copy the link below to verify your account:
				${url.toString()}
			`,
			html: `<h2>Verify your token!</h2>
			<p>Was this not you? Then please ignore this message, or let us know by sending a reply.</p>
			<p>Please click or copy the link below to verify your account:</p>
			<a href="${url.toString()}">Verify your account!</a>
			`,
		});
	
		return This.send({ message: `Verification E-mail sent to ${data.ok.email}! (It may be in your spam folder!)` });
	}
	
	return This.send({ message: `Successfully registered!`, token: result.ok.jwt, username: result.ok.username });
}