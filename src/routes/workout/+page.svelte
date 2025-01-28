<script lang="ts">
    import { Button, Root } from "$lib/components/ui/button";
    import { Label } from "$lib/components/ui/label"
    import NumberInput from "$lib/components/NumberInput.svelte";
    import { Checkbox } from "$lib/components/ui/checkbox"
    import * as Carousel from "$lib/components/ui/carousel";

    import {
        AlertDialog,
        AlertDialogAction,
        AlertDialogCancel,
        AlertDialogContent,
        AlertDialogDescription,
        AlertDialogFooter,
        AlertDialogHeader,
        AlertDialogTitle,
        AlertDialogTrigger,
    } from "$lib/components/ui/alert-dialog"
    import {
        Card,
        CardContent,
        CardDescription,
        CardFooter,
        CardHeader,
        CardTitle,
    } from "$lib/components/ui/card"

    const urlParams = new URLSearchParams(window.location.search);
    let workoutName:string = urlParams.get("name")??"Error";

    interface Exercise {
        name: String,
        description: String,
        steps: String[],
        done?:boolean,
        reps?:number,
        weight?:number
    }

    interface databaseWorkout {
        exercises: String[],
        name: string,
        description: string
    }

    interface Workout {
        exercises: Exercise[],
        name: string,
        description: string
    }

    //@ts-ignore
    const invoker = window.__TAURI__.core.invoke;

    async function get_workout_exercises(): Promise<Exercise[]> {
        let resp = await invoker("get_workout_exercises", { name:workoutName });
        return resp as Exercise[]
    }

    async function get_workout_data():Promise<databaseWorkout> {
        let resp = await invoker("get_workout", { name:workoutName });

        return resp as databaseWorkout
    }


    let unfinishedEx:boolean = $state(false);
    let alertDismissed:boolean = $state(false);

    let workoutData:Workout;
    let loaded:boolean = $state(false);

    get_workout_data().then((resp:databaseWorkout)=>{
        get_workout_exercises().then((exercises:Exercise[])=>{
            exercises.forEach(el=>{
                el.done = false;
            });
            workoutData = {
            name: resp.name,
            description: resp.description,
            exercises: exercises
        };
        console.log(workoutData.exercises);
        loaded = true;
        })
    });

    function saveWorkout() {
        console.log("savinf workout");
        workoutData.exercises.forEach((exercise:Exercise) => {
            if(!exercise.done && !alertDismissed) {
                unfinishedEx = true;
                throw Error("Waiting for user confirmation");
            }
        });

        console.log(workoutData.exercises);

        invoker("save_workout", {
            name:workoutName,
            exercises: workoutData.exercises,
            date: Math.round(Date.now() / 1000)
            }
        );
        
        alertDismissed = false;
        console.log("Saved workout");

    }

</script>

<div class="workout ml-[auto] mr-[auto] mt-[auto] flex flex-col justify-center">
    {#if loaded}
        <h1 class="text-4xl font-extrabold text-left ml-[5vw]">{workoutData.name}</h1>
        <p class="text-3xl font-semibold text-left ml-[5vw]" id="workout-description">{workoutData.description}</p>
        <br>
        <hr>

        {#each workoutData.exercises as exercise}
            <Card class="w-[90vw] ml-[auto] mr-[auto] p-[none]">
                <CardHeader>
                    <div class="flex items-center space-x-2">
                        <Checkbox on:click={()=>{exercise.done = !exercise.done}} id="done"></Checkbox>
                        <Label htmlFor="done" class="exercise-name text-3xl {exercise.done}">{exercise.name}</Label>
                    </div>
                    <p class="description mb-[4px]">{exercise.description}</p>
                </CardHeader>
                <CardContent>
                    <h3 class="text-xl font-semibold mb-[10px]">Steps:</h3>
                    <Carousel.Root>
                        <Carousel.Content>
                        {#each exercise.steps as step, index}
                            <Carousel.Item key={index} class="basis-1/1.5">
                                <Card>
                                    <CardContent>
                                        <span on:click={()=>{console.log("hii")}}>{index+1}. {step}</span>
                                    </CardContent>
                                </Card>
                            </Carousel.Item>
                        {/each}
                        </Carousel.Content>
                    </Carousel.Root>
                </CardContent>
                <CardFooter>
                    <div class="flex-row">
                        <h3 class="text-xl mb-[10px]">Reps:</h3>
                        <NumberInput bind:value={exercise.reps} inputClass="w-[10ch]"/>
                    </div>
                    <div class="flex-row">
                        <h3 class="text-xl mb-[10px]">Weight:</h3>
                        <NumberInput bind:value={exercise.weight} inputClass="w-[10ch]"/>
                    </div>
                </CardFooter>
            </Card>
            <br>
        {/each}
    {/if}

    <Button
        class="mb-[20vh] mt-[5vh] h-[10vh] w-[80vw] ml-[auto] mr-[auto]"
        on:click={()=>saveWorkout()}
    >Finish workout!</Button>


    <AlertDialog open={unfinishedEx}>
        <AlertDialogContent>
            <AlertDialogHeader>
                <AlertDialogTitle>Are you sure you want to finish the workout?</AlertDialogTitle>
                <AlertDialogDescription>
                    You haven't completed all of the exercises in this workout.
                </AlertDialogDescription>
            </AlertDialogHeader>
            <AlertDialogFooter>
                <AlertDialogCancel>Cancel</AlertDialogCancel>
                <AlertDialogAction on:click={()=>{alertDismissed=true;saveWorkout()}}>Finish</AlertDialogAction>
            </AlertDialogFooter>
        </AlertDialogContent>
    </AlertDialog>


</div>

<style>
    ol {
        list-style:circle;
        counter-reset: step;
    }
    li {
        counter-increment: step;
    }
    li:before {
        margin-right: 0.5em;
        content: counter(step)
    }
    #workout-description, .description {
        font-style: italic;
    }

    .description::before {
        content: '"'
    }
    .description::after {
        content: '"'
    }

    :global(.exercise-name) {
        text-transform: capitalize;
    }

    :global(.checked) {
        text-decoration: overline;
    }

</style>