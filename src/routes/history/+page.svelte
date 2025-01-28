<script lang="ts">
  import { Button } from "$lib/components/ui/button";
  import { Calendar } from "$lib/components/ui/calendar";
  import { Area, Axis, Chart, Highlight, Svg, Tooltip } from "layerchart";
  import {
    Dialog,
    DialogContent,
    DialogDescription,
    DialogFooter,
    DialogHeader,
    DialogTitle,
    DialogTrigger,
  } from "$lib/components/ui/dialog";
  import { scaleTime } from "d3-scale";
  import { today, getLocalTimeZone } from "@internationalized/date";

  let rawData = [];
  let dayAmount: number = 70;
  for (let i = 0; i < dayAmount; i++) {
    rawData.push({
      date: Date.now() / 1000 - dayAmount * 86400 + i * 86400,
      value: Math.round(Math.random() * 20 * i),
    });
  }

  let data = processData(1737622063);

  function processData(from?: Number) {
    let finalData = [];
    rawData.forEach((d) => {
      if (!from) {
        finalData.push({ date: new Date(d.date*1000), value: d.value });
      } else if (d.date > from) {
        finalData.push({ date: new Date(d.date*1000), value: d.value });
      }
    });

    return finalData;
  }

  let date = today(getLocalTimeZone());

  $: data = processData(date.toDate(getLocalTimeZone()).getTime()/1000);

  $: console.log(data);
</script>

<h1 class="text-5xl font-extrabold">Viewing stats for</h1>

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
          <Tooltip.Header>{new Date(data.date).toUTCString()}</Tooltip.Header>
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
