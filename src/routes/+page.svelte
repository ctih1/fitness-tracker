<script lang="ts">
    import { onMount } from "svelte"
    import Reload from "svelte-radix/Reload.svelte"

    import { Button } from "$lib/components/ui/button"
    import { Input } from "$lib/components/ui/input"
    import { Label } from "$lib/components/ui/label"
    import { Textarea } from "$lib/components/ui/textarea"

    import {
	    blur,
        slide
    } from 'svelte/transition'
    

    import {
        Dialog,
        DialogContent,
        DialogDescription,
        DialogFooter,
        DialogHeader,
        DialogTitle,
    } from "$lib/components/ui/dialog"

    import {
        Card,
        CardContent,
        CardDescription,
        CardFooter,
        CardHeader,
        CardTitle,
    } from "$lib/components/ui/card"



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

    interface Exercise {
        description: String,
        steps: String
        name: String,
        selected: boolean
    }


    let workouts:customWorkout[] = [];
    let exercises:Exercise[];

    let name:String="";
    let description:String="";
    let steps:String="";

    let workoutName:String="";
    let workoutDescription:String="";

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

    async function createExercise() {
        let step_array = steps.split("\n");
        await invoker("create_exercise", {name, description, steps:step_array})
        .then(()=>{
            window.location.reload();
        })
    }

    async function createWorkout() {
        let selectedWorkouts:String[] = [];
        exercises.forEach((e:Exercise)=>{
            if(e.selected) {
                selectedWorkouts.push(e.name);
            };
        });
        await invoker("create_workout", {name:workoutName, description:workoutDescription, exercises: selectedWorkouts});
    }

    async function getExercises() {
        let exercises = await invoker("get_every_exercise");
        console.log(exercises);   
        //@ts-ignore
        exercises.forEach(e=>{
            e["selected"] = false;
        });
        return exercises;
    }


    let creatingExercise = false;
    let creatingWorkout = false;
    let newWorkout = false;
    
    //@ts-ignore
    function startWorkout(name:string, button) {
        console.log(button);
        button.disabled = true;
        button.variant="loading";
        window.location.href=`/workout?name=${name}`;
    }

    
    onMount(()=>{
        getWorkouts().then(response=>{
            workouts = response;
        });

        getExercises().then(e=>{
            exercises = e;
        });
    })
</script>

<div>
    <h1 class="scroll-m-20 text-7xl font-extrabold">New workout</h1>
    <p class="font-bold">You can easily start new workouts here</p>
    <div class="wrapper w-100 h-[60vh] flex justify-center items-center">
        <div class="buttons">
            <Button class="w-[50vw] h-[10vh]" on:click={()=>newWorkout=true}>Start new workout!</Button>    
            <div class="secondary flex mt-[0.5em]">
                <Button class="w-[50%] m-[0.5em] ml-[0px]" variant="secondary" on:click={()=>creatingExercise=true}>Create exercise</Button>   
                <Button class="w-[50%] m-[0.5em] mr-[0px]" variant="outline" on:click={()=>creatingWorkout=true}>Create workout</Button>    

            </div>

        </div>
        <Dialog open={newWorkout}>
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
        <DialogContent>
            <DialogHeader>
                <DialogTitle>Create exercise</DialogTitle>
                <DialogDescription>Create a new excercise. Press save to save</DialogDescription>
            </DialogHeader>
            <div>
                <div class="basic-info">
                    <Label>Name</Label>
                    <Input placeholder="Squat" bind:value={name}/>
                </div>
                <div class="class-info">
                    <Label>Description</Label>
                    <Input placeholder="A squat is a lower-body exercise where you bend your knees and hips to lower your body, then return to a standing position, targeting the legs and glutes." bind:value={description}/>
                </div>
                <div class="class-info">
                    <Label>Steps</Label>
                    <Textarea placeholder="Stand with feet shoulder-width apart.
Bend your knees and lower your body like you're sitting.
Keep your back straight and chest up.
Push through your heels to stand back up." 
                    bind:value={steps} class="resize-y min-h-[10em]" 
                    />
                </div>
            </div>
            <DialogFooter>
                <Button on:click={()=>{creatingExercise=false;createExercise()}} type="submit">Save</Button>
            </DialogFooter>
        </DialogContent>
    </Dialog>

        <Dialog open={creatingWorkout}>
            <DialogContent class="h-[80vh] overflow-y-scroll">
                <DialogHeader>
                    <DialogTitle>Create workout</DialogTitle>
                    <DialogDescription>Create new workout with exercises you've created</DialogDescription>
                    
                    <div class="selected flex">
                        {#each exercises as exercise}
                            {#if exercise.selected}
                                <p>{exercise.name}</p>
                                <Button class="" on:click={()=>exercise.selected=false}>X</Button>
                            {/if}
                        {/each}
                    </div>
                    
                </DialogHeader>
                <div>
                    <Label>Workout name</Label>
                    <Input bind:value={workoutName} />
                    <Label>Workout description</Label>
                    <Input bind:value={workoutDescription} />
                </div>
                <div>
                    <h1>Select exercises to include</h1>
                    {#each exercises as exercise}
                        {#if !exercise.selected}
                        <div in:blur out:slide={{ duration: 500 }} class="wrap">
                            <Card>
                                <CardHeader>
                                    <h1 class="font-extrabold text-3xl">{exercise.name}</h1>
                                    <p>{exercise.description}
                                </CardHeader>
                                <CardContent>
                                    <Label class="font-semibold text-xl">Steps: </Label>
                                    <ol>
                                    {#each exercise.steps as step, index}
                                        <li>{index+1}. {step}</li>
                                    {/each}
                                    </ol>
                                </CardContent>
                                <CardFooter>
                                    <Button on:click={()=>{exercise.selected=true}} variant="secondary">Add</Button>
                                </CardFooter>   
                            </Card>
                        </div>
                        {/if}
                    {/each}
                </div>
                <DialogFooter>
                    <Button on:click={()=>{creatingWorkout=false; createWorkout()}} type="submit">Save</Button>
                </DialogFooter>
            </DialogContent>
        </Dialog>
</div>

<style>
    .selected p:not(:last-child)::after{
        content: ",";
    }
</style>