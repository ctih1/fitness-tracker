<script>
	import '../app.css';
	import { ModeWatcher } from "mode-watcher";
	import { TabsList,Tabs, TabsTrigger } from "$lib/components/ui/tabs";
    import { resetMode, setMode } from "mode-watcher";

	let { children } = $props();
	document.addEventListener('contextmenu', event => event.preventDefault());
	console.log(window.location.pathname);
	
</script>
<ModeWatcher />

{@render children()}
<div class="tabs fixed bottom-0 ml-a mr-a w-screen">
	<Tabs on:change={(val)=>console.log(val)}  value={window.location.pathname}>
		<TabsList class="w-screen h-[10vh]">
			<TabsTrigger class="h-[9vh] w-[25vw] text-xl" on:click={()=>window.location.href="/history"} value="/history">History</TabsTrigger>
			<TabsTrigger class="h-[9vh] w-[25vw] text-xl" on:click={()=>window.location.href="/"} value="/">New</TabsTrigger>
			<TabsTrigger class="h-[9vh] w-[25vw] text-xl" on:click={()=>window.location.href="/settings"} value="/settings">Settings</TabsTrigger>
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