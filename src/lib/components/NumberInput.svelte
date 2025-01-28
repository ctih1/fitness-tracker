<script lang="ts">
    import { Button, Root } from "$lib/components/ui/button";
    import { Label } from "$lib/components/ui/label"
    import { Input } from "$lib/components/ui/input";
    import { Slider } from "$lib/components/ui/slider"

    let l_value:number=0;
    export let value:number = l_value;
    export let inputClass:string;

</script>

<div class="wrapper flex">
    <Button class="h-[7vh] aspect-square" on:click={()=>l_value > 0? l_value--:l_value}  variant="secondary">-</Button>
    <Input min="0" bind:value={l_value} class="focus-visible:ring-transparent h-[7vh] w-[100%] text-3xl text-right" type="number" />
    <Button class="h-[7vh] aspect-square" on:click={()=>l_value++} variant="secondary">+</Button>
</div>
