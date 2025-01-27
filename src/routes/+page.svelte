<script lang="ts">
    import { Button } from "$lib/components/ui/button";
    import Reload from "svelte-radix/Reload.svelte";
    import {
        Dialog,
        DialogContent,
        DialogDescription,
        DialogFooter,
        DialogHeader,
        DialogTitle,
        DialogTrigger,
    } from "$lib/components/ui/dialog"

    import {
        Card,
        CardContent,
        CardDescription,
        CardFooter,
        CardHeader,
        CardTitle,
    } from "$lib/components/ui/card";
    import { Input } from "$lib/components/ui/input"
    import { Label } from "$lib/components/ui/label"
    import { invoke } from '@tauri-apps/api/core';
    import { onMount } from "svelte";

    //@ts-ignore
    const invoker = window.__TAURI__.core.invoke;

    interface customWorkout {
        name: string,
        description: string,
        items: number,
        approxTime: number,
        loading?: boolean
    }

    interface databaseWorkout {
        exercises: Map<String,String[]>,
        name: string,
        description: string
    }

    let workouts:customWorkout[] = []

    onMount(()=>{
        getWorkouts().then(response=>{
            workouts = response;
        })
    })

    async function getWorkouts(): Promise<customWorkout[]> {
        let workouts:databaseWorkout[] = await invoker("get_every_workout");
        let calculatedWorkouts:customWorkout[] = [];

        workouts.forEach((workout:databaseWorkout)=>{
            calculatedWorkouts.push({
                name: workout.name,
                description: workout.description,
                items: workout.exercises.size,
                approxTime: workout.exercises.size*3
            })
        });

        return calculatedWorkouts;
        
    }

    async function getExercises() {
        let exercises = await invoker("get_every_exercise");
        console.log(exercises);   
        return exercises;
    }


    let creatingExercise = false;
    let newWorkout = false;

    async function greet() {
        let resp = await invoker('save_exercise', { name:basicInfo, classes });
        console.log(resp);
        creatingExercise = false;
    }

    
    //@ts-ignore
    function startWorkout(name:string, button) {
        console.log(button);
        button.disabled = true;
        button.variant="loading";
        window.location.href=`/workout?name=${name}`;
    }

    getExercises();

    let basicInfo:String;
    let classes:String;
</script>

<div>
    <h1 class="scroll-m-20 text-7xl font-extrabold">New workout</h1>
    <p class="font-bold">You can easily start new workouts here</p>
    <div class="wrapper w-100 h-[80vh] flex justify-center items-center">
        <Dialog open={newWorkout}>
            <DialogTrigger>
                <Button on:click={()=>newWorkout=true}>Start new workout!</Button>    
            </DialogTrigger>
            <DialogContent>
                <DialogHeader>
                    <DialogTitle>Start a new workout</DialogTitle>
                    <DialogDescription>Start either a saved workout, or create a new one</DialogDescription>
                </DialogHeader>
                <div class="h-[60vh] overflow-y-scroll">
                    <h1>Available workouts: </h1>
                    {#each workouts as workout}
                        <Card>
                            <CardHeader>
                                <h1 class="text-2xl font-extrabold">{workout.name}</h1>
                                <p>{workout.description}</p>
                            </CardHeader>
                            <CardContent>
                                <span>Items: {workout.items}</span>
                                <span>Estimated time: {workout.approxTime} minutes</span>
                            </CardContent>
                            <CardFooter>
                                <Button on:click={(event)=>{workout.loading=true; startWorkout(workout.name,event.target)}}>
                                    {#if workout.loading??false}
                                        <Reload class="mr-2 h-4 w-4 animate-spin"/>
                                    {/if}
                                    Start!
                                </Button>
                                <Button class="ml-[auto] mr-0" variant="destructive">Delete</Button>
                            </CardFooter>
                        </Card>

                    {/each}
                </div>
                <DialogFooter>
                    <Button on:click={()=>newWorkout=false} variant="outline" type="submit">Cancel</Button>
                </DialogFooter>
            </DialogContent>
        </Dialog>
    </div>
    <Dialog open={creatingExercise}>
        <DialogTrigger>
            <Button variant="outline" on:click={()=>creatingExercise=true}>Create exercise</Button>    
        </DialogTrigger>
        <DialogContent>
            <DialogHeader>
                <DialogTitle>Create exercise</DialogTitle>
                <DialogDescription>Create a new excercise. Press save to save</DialogDescription>
            </DialogHeader>
            <div>
                <div class="basic-info">
                    <Label>Name</Label>
                    <Input bind:value={basicInfo}/>
                </div>
                <div class="class-info">
                    <Label>Type</Label>
                    <Input bind:value={classes}/>
                </div>
            </div>
            <DialogFooter>
                <Button on:click={greet} type="submit">Save</Button>
            </DialogFooter>
        </DialogContent>
    </Dialog>
</div>

