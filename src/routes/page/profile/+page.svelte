<script lang="ts">
	import * as Tabs from "$lib/shadcn/ui/tabs";
	import * as Register from "$api/auth/register/endpoints.js";
	import * as Login from "$api/auth/login/endpoints.js";
	import AppBounds from '$lib/components/AppBounds.svelte';
	import RegisterForm from "./RegisterForm.svelte";
	import type {SuperValidated} from "sveltekit-superforms";
	import Dock from "$lib/components/Dock.svelte";
	import {client_auth} from "$api/auth";
	import LoginForm from "./LoginForm.svelte";
	import H1 from "$lib/components/H1.svelte";
	import {Button} from "$lib/shadcn/ui/button";
	import Image from "svimg/Image.svelte";
	import { env } from '$env/dynamic/public';
	import Meta from '$lib/components/Meta.svelte';

	export let register_form: SuperValidated<typeof Register.User>;
	export let login_form: SuperValidated<typeof Login.User>;
	let title = $client_auth?.username ?? "Login / Register";
	
	function onAuthenticate({token, username}: {message: string, token: string, username: string}) {
		localStorage.setItem("jwt", token);
		localStorage.setItem("username", username);
		client_auth.set({ jwt: token, username });
		title = username;
	}
</script>

<Meta pageTitle={title}/>

<AppBounds>
	{#if $client_auth != null}
		<Dock>
			<H1>Logged in as {$client_auth.username}</H1>
			<Button class="rounded-xl w-full" on:click={() => {
				localStorage.removeItem("jwt");
				localStorage.removeItem("username");
				client_auth.set(null);
			}}>Log out</Button>
		</Dock>
	{:else}
		<Dock class="pt-4">
			<Tabs.Root value="login" class="">
				<Tabs.List class="rounded-xl">
					<Tabs.Trigger class="rounded-xl" value="login">Log In</Tabs.Trigger>
					<Tabs.Trigger class="rounded-xl" value="register">Register</Tabs.Trigger>
				</Tabs.List>
				<Tabs.Content value="register">
					<Image src="/Joco-02.png" alt="Joco" class="w-full"/>
					<RegisterForm form={register_form} {onAuthenticate}/>
				</Tabs.Content>
				<Tabs.Content value="login">
					<Image src="/Joco-02.png" alt="Joco" class="w-full"/>
					<LoginForm form={login_form} {onAuthenticate}/>
				</Tabs.Content>
			</Tabs.Root>
		</Dock>
	{/if}
</AppBounds>
