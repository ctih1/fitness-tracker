<script>
	import '../app.css';
	import { ModeWatcher } from "mode-watcher";
	import { TabsList,Tabs, TabsTrigger } from "$lib/components/ui/tabs";
    import { blur, fade, fly, scale } from 'svelte/transition';
    import { onMount } from 'svelte';
    import { expoIn, expoOut, quartIn } from 'svelte/easing';
    import { onNavigate } from '$app/navigation';

	let { children } = $props();
	document.addEventListener('contextmenu', event => event.preventDefault());
	let loaded = $state(false);

	onMount(()=>{
		loaded=true;
	})
</script>
<ModeWatcher />

{#if loaded}
<div in:blur={{duration: 300, easing: expoOut}} out:fade class="wrapper">
	{@render children()}
</div>
{/if}
<div class="tabs fixed bottom-0 ml-a mr-a w-screen">
	<Tabs value={window.location.pathname}>
		<TabsList class="w-screen h-[10vh]">
			<TabsTrigger class="h-[9vh] w-[25vw] text-xl" on:click={()=>{loaded=false;window.location.href="/history"}} value="/history">History</TabsTrigger>
			<TabsTrigger class="h-[9vh] w-[25vw] text-xl" on:click={()=>{loaded=false;window.location.href="/"}} value="/">New</TabsTrigger>
			<TabsTrigger class="h-[9vh] w-[25vw] text-xl" on:click={()=>{loaded=false;window.location.href="/settings"}} value="/settings">Settings</TabsTrigger>
		</TabsList>
	</Tabs>
</div>


<style>

	@import url('https://fonts.googleapis.com/css2?family=Inter:ital,opsz,wght@0,14..32,100..900;1,14..32,100..900&display=swap');
	:root {
		touch-action: pan-x pan-y;
		height: 100%
	}
	:global(*) {
		font-family: "Inter", serif !important;
	}
</style>