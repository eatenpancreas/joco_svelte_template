import * as This from './endpoints';

export async function GET() {
	
	return This.GET.send({ message: "Welcome, This is the Joco Svelte Template!" });
}