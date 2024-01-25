import { createTransport } from 'nodemailer';
import { env } from '$env/dynamic/private';

export default createTransport({
	host: env.EMAIL_HOST,
	port: env.EMAIL_PORT,
	auth: {
		user: env.EMAIL_USER,
		pass: env.EMAIL_PASS,
	},
});