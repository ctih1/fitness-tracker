<script lang="ts">
    import { Area, Axis, Chart, Highlight, Svg, Tooltip } from "layerchart";
    import { scaleTime } from "d3-scale";
    import { today, getLocalTimeZone } from "@internationalized/date";

    import { Button } from "$lib/components/ui/button";
    import { Calendar } from "$lib/components/ui/calendar";

    import * as Carousel from "$lib/components/ui/carousel";

    import {
        Card,
        CardContent
    } from "$lib/components/ui/card";

    import {
        Dialog,
        DialogContent,
        DialogDescription,
        DialogFooter,
        DialogHeader,
        DialogTitle,
        DialogTrigger,
    } from "$lib/components/ui/dialog";
    import { blur, fly } from "svelte/transition";

    
    //@ts-ignore
    const invoker = window.__TAURI__.core.invoke;

    interface WorkoutHistory {
        name:String,
        exercises:{name:String,reps:number,weight:number}[],
        date:number
    }

    interface ChartData {
        date: Date,
        value: number;
    }

    //@ts-ignore
    let workoutHistory:Map<String,WorkoutHistory[]> = $state();
    let workoutChartData:ChartData[];
    //@ts-ignore
    let timeChartData:ChartData[] = $state({date:new Date(), value:0});

    let historyLoaded:boolean = $state(false);
    let statsOpened:boolean = $state(false);

    function processData(data:ChartData[], from: Date) {
        return data.filter((d:ChartData) => {
            return d.date.getTime() > from.getTime()
        });
    }

    function processWorkoutHistory(wh:any):ChartData[] {
        let data:ChartData[] = [];
        wh.forEach((key:WorkoutHistory[],value:string) => {
            let repsForDay:number = 0;
            key.forEach((workout:WorkoutHistory) => {
                workout.exercises.forEach((item)=>{
                    repsForDay += item.reps;
                });
            });
            let [day, month, year] = value.split("-");
            let date = new Date(`${year}-${month}-${day}`);
            data.push({date,value:repsForDay});

        });
        return data;
    };

    let date = $state(today(getLocalTimeZone()));

    //@ts-ignore
    invoker("get_history").then(r=>{
        workoutHistory = new Map(Object.entries(r)); 
        historyLoaded = true;
        workoutChartData = processWorkoutHistory(workoutHistory);
        
    });

    $effect(()=>{
        if(historyLoaded) {
            timeChartData = processData(workoutChartData, date.toDate(getLocalTimeZone()));
        } 
    });
</script>

<h1 class="text-5xl font-extrabold">History</h1>

<Dialog open={statsOpened}>
    <DialogTrigger>
        <Button on:click={()=>statsOpened=true} variant="outline">View stats</Button>
    </DialogTrigger>
    <DialogContent class="w-[90vw]">
        <DialogHeader>
            <DialogTitle>History of Exercise</DialogTitle>
            <DialogDescription
                >Create a new excercise. Press save to save</DialogDescription
            >
        </DialogHeader>
        <Calendar bind:value={date} />
        <div class="h-[40vh] p-4 border rounded">
            <Chart
                data={timeChartData}
                x="date"
                xScale={scaleTime()}
                y="value"
                yDomain={[0, null]}
                yNice
                padding={{ left: 16, bottom: 24 }}
                tooltip={{ mode: "bisect-x" }}
            >
                <Svg>
                    <Axis placement="left" grid rule />
                    <Axis placement="bottom" rule />
                    <Area
                        line={{ class: "stroke-[2px] stroke-violet-950" }}
                        class="fill-violet-400"
                    />
                    <Highlight points lines />
                </Svg>
                <Tooltip.Root let:data>
                    <Tooltip.Header
                        >{data.date.toISOString().split("T")[0]}</Tooltip.Header
                    >
                    <Tooltip.List>
                        <Tooltip.Item label="Reps" value={data.value} />
                    </Tooltip.List>
                </Tooltip.Root>
            </Chart>
        </div>

        <DialogFooter>
            <Button on:click={()=>statsOpened=false} type="submit">Close</Button>
        </DialogFooter>
    </DialogContent>
</Dialog>

{#if historyLoaded}
<div in:fly class="wrapper">
    <Carousel.Root opts={{direction:"rtl"}}>
        <Carousel.Content dir="rtl">
            {@const dates = Array.from(workoutHistory.keys())}
            {#each dates as date}
                {@const workoutData = workoutHistory.get(date) || []}
                <Carousel.Item>
                    <Card class="w-[80vw] mr-[auto] ml-[auto]">
                        <h1 class="text-4xl font-extrabold text-center">{date}</h1>
                        <hr>

                        {#each workoutData as workout, index}
                            <CardContent dir="ltr" class="flex flex-col justify-center text-center">
                                <h1 class="text-2xl font-bold">{workout.name}</h1>
                                <div class="details text-left">
                                    <p class="text-xl font-semibold">Exercises</p>
                                    <ul>
                                        {#each workout.exercises as exercise}
                                            <li>{exercise.name}</li>
                                            <li class="pl-[2em]">Reps: {exercise.reps}</li>
                                            <li class="pl-[2em]" >Weight: {exercise.weight}</li>
                                        {/each}
                                    </ul>
                                </div>
                            </CardContent>
                            {#if index != dates.length+1}
                            <hr>
                            {/if}
                        {/each}
                    </Card>
                </Carousel.Item>
            {/each}
        </Carousel.Content>

        <Carousel.Previous/>
        <Carousel.Next/>
    </Carousel.Root>
</div>
{/if}

