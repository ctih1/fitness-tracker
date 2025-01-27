<script>
    import { Button } from "$lib/components/ui/button";
    import { TabsList,Tabs, TabsTrigger } from "$lib/components/ui/tabs";
    import { resetMode, setMode } from "mode-watcher";
</script>



<Button on:click={()=>setMode("dark")}>Enable dark mode</Button>
<Button on:click={()=>setMode("light")}>Enable light mode</Button>

