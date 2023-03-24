import { Show } from "solid-js";

const CurrentStation = (props) => {
  return (
    <div class="col-span-4 flex space-x-2">
      <div class="min-w-fit">
        <Show when={props.station.favicon && props.station.favicon.length > 0} fallback={<div class="h-20 w-20"></div>}>
          <img src={props.station.favicon} class="h-20 w-20" />
        </Show>
      </div>
      <div class="flex flex-col truncate justify-center">
        <span class="font-bold text-ellipsis truncate">{props.station.name}</span>
        <a target="_blank" href={props.station.homepage} class="text-slate-600 text-ellipsis truncate hover:text-slate-400 text-sm">{props.station.homepage}</a>
      </div>
    </div>
  );
}

export default CurrentStation;
