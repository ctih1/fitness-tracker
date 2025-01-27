<script lang="ts">
    import { Button, Root } from "$lib/components/ui/button";
    import { Label } from "$lib/components/ui/label"
    import { Input } from "$lib/components/ui/input";
    import { Slider } from "$lib/components/ui/slider"

    export let value:number=0;
    export let inputClass:string;

</script>

<div class="wrapper flex">
    <Button class="h-[7vh] aspect-square" on:click={()=>value > 0? value--:value}  variant="secondary">-</Button>
    <Input min="0" bind:value={value} class="focus-visible:ring-transparent h-[7vh] w-[100%] text-3xl text-right" type="number" />
    <Button class="h-[7vh] aspect-square" on:click={()=>value++} variant="secondary">+</Button>
</div>
