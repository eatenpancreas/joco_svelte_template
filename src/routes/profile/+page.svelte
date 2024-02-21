<script lang="ts">
	import * as Tabs from "$lib/shadcn/ui/tabs";
	import AppBounds from '$lib/components/AppBounds.svelte';
	import RegisterForm from "./RegisterForm.svelte";
	import Dock from "$lib/components/Dock.svelte";
	import Meta from '$lib/components/Meta.svelte';
	import type { SuperValidated } from 'formsnap';
	import type { LoginUser, RegisterUser } from './auth_commons';
	import user from '$lib/http/user';
	import H1 from '$lib/components/H1.svelte';
	import { Button } from '$lib/shadcn/ui/button';
	import LoginForm from './LoginForm.svelte';
	import { onMount } from 'svelte';

	export let register_form: SuperValidated<RegisterUser>;
	export let login_form: SuperValidated<LoginUser>;
	let title = "Login / Register";
	let username: string | undefined;
	let is_mounted = false;
	
	onMount(() => {
		is_mounted = true;
	});
	
	user.subscribe((value) => {
		if (value && is_mounted) {
			localStorage.setItem("username", value.username);
		} else if (is_mounted) {
			localStorage.removeItem("username");
		}
		username = value?.username;
	});
</script>

<Meta pageTitle={title}/>

<AppBounds>
	{#if username}
		<Dock>
			<H1>Logged in as {username}</H1>
			<Button class="rounded-xl w-full" on:click={() => {
				user.set(undefined);
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
					<img src="/Joco-02.svg" alt="Joco" class="w-full"/>
					
					<RegisterForm form={register_form}/>
				</Tabs.Content>
				<Tabs.Content value="login">
					<img src="/Joco-02.svg" alt="Joco" class="w-full"/>
					<LoginForm form={login_form}/>
				</Tabs.Content>
			</Tabs.Root>
		</Dock>
	{/if}
</AppBounds>
