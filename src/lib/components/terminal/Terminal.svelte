<script lang="ts">
	import { AspectRatio } from '$lib/shadcn/ui/aspect-ratio';
	import { onDestroy, onMount } from 'svelte';
	import * as Terminal from '$api/files/terminal/endpoints';
	
	let history: { directory: string, in_: string, out: string }[] = [];
	let currentText = '';
	let main_directory = 'MC server/'
	let directory = ""
	let cursor = "";
	let timeout = 0;
	let focused = false;
	
	function handleType(event: any) {
		currentText = event.target.value;
		console.log(event.target.value);
	}
	async function handleSubmit(event: any) {
		if (event.key === 'Enter') {
			event.preventDefault();
			if (timeout > 0) return;

			const result = await Terminal.GET.fetch({ input: currentText, directory: "" });
			let message = result.ok?.output ?? result.err?.message ?? "Unknown error";
			
			history = [...history, { directory: main_directory + directory, in_: currentText, out: message }];
			currentText = '';
			
			document.querySelector('#terminal').value = "";
			
			timeout = 4;
			
			setTimeout(() => {
				document.querySelector('#history')?.scrollTo(0, 100000);
			}, 50);
		}
		if (event.key === 'ArrowLeft' || event.key === 'ArrowRight') {
			event.preventDefault();
		}
	}

	let clear: any;
	$: {
		clearInterval(clear)
		clear = setInterval(() => {
			timeout -= 1;
			if (cursor === "") {
				cursor = focused? "|" : "";
			} else {
				cursor = "";
			}
		}, 500)
	}
	
	function focusTerminal() {
		document.querySelector('#terminal')?.focus();
	}
	
	onMount(() => {
		focusTerminal();
	});
	
	function rebuildOutputNewlines(output: string) {
		return "<p>" + output.replaceAll("\n", "</p><p>") + "</p>";
	}
</script>

<AspectRatio ratio={16 / 9} class="w-full bg-gradient rounded-2xl">
	<div id="history" role="button" tabindex="0" 
			 on:seeking={focusTerminal} on:keydown={focusTerminal} 
			 on:focus={focusTerminal} on:click={focusTerminal} on:touchstart={focusTerminal}
			 aria-label="select-textarea" aria-keyshortcuts="ArrowLeft,ArrowRight,Enter" 
			 class="absolute inset-0 z-10 -right-4 text-secondary p-5 overflow-scroll">
		
			{#each history as { directory, in_, out }}
				<div class="font-bold border-t border-b border-secondary">{directory} > {in_}</div>
				<div>{@html rebuildOutputNewlines(out)}</div>
			{/each}
			<div class="font-bold border-t border-b border-secondary">{main_directory + directory} > {currentText}{cursor}</div>
	</div>
	<textarea id="terminal" class="resize-none bg-transparent rounded-2xl h-full w-full caret-transparent text-transparent" 
						on:input={handleType} on:keydown={handleSubmit} 
						on:focusin={() => focused = true} on:focusout={() => focused = false}></textarea>
</AspectRatio>