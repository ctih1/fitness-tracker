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

    
    //@ts-ignore
    const invoker = window.__TAURI__.core.invoke;

    interface WorkoutHistory {
        name:String,
        exercises:{name:String,reps:number,weight:number}[],
        date:number
    }

    interface ChartData {
        date: Date,
        value: string
    }

    let workoutHistory:Map<String,WorkoutHistory[]>;

    let rawData:any[] = [];
    let dayAmount: number = 70;
    let historyLoaded:boolean = false

    for (let i = 0; i < dayAmount; i++) {
        rawData.push({
            date: Date.now() / 1000 - dayAmount * 86400 + i * 86400,
            value: Math.round(Math.random() * 20 * i),
        });
    }

    let data = processData(1737622063);

    function processData(from?: Number) {
        let finalData:ChartData[] = [];
        rawData.forEach((d) => {
            if (!from) {
                finalData.push({
                    date: new Date(d.date * 1000),
                    value: d.value,
                });
            } else if (d.date > from) {
                finalData.push({
                    date: new Date(d.date * 1000),
                    value: d.value,
                });
            }
        });

        return finalData;   
    }

    //@ts-ignore
    invoker("get_history").then(r=>{
        workoutHistory = new Map(Object.entries(r)); 
        historyLoaded = true;
    });

    let date = today(getLocalTimeZone());
    $: data = processData(date.toDate(getLocalTimeZone()).getTime() / 1000);  
</script>

<h1 class="text-5xl font-extrabold">History</h1>

{#if historyLoaded}
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
{/if}

<Dialog>
    <DialogTrigger>
        <Button variant="outline">View stats</Button>
    </DialogTrigger>
    <DialogContent class="w-[90vw]">
        <DialogHeader>
            <DialogTitle>History of Exercise</DialogTitle>
            <DialogDescription
                >Create a new excercise. Press save to save</DialogDescription
            >
        </DialogHeader>
        <Calendar bind:value={date} />
        <div class="h-[300px] p-4 border rounded">
            <Chart
                {data}
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
                        >{new Date(data.date).toUTCString()}</Tooltip.Header
                    >
                    <Tooltip.List>
                        <Tooltip.Item label="value" value={data.value} />
                    </Tooltip.List>
                </Tooltip.Root>
            </Chart>
        </div>

        <DialogFooter>
            <Button type="submit">Close</Button>
        </DialogFooter>
    </DialogContent>
</Dialog>
