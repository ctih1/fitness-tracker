<script>
    import { Button } from "$lib/components/ui/button";
    import { setMode } from "mode-watcher";
</script>



<Button on:click={()=>setMode("dark")}>Enable dark mode</Button>
<Button on:click={()=>setMode("light")}>Enable light mode</Button>

