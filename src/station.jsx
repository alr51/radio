import { invoke } from "@tauri-apps/api";
import { HiSolidPlay, HiOutlineHeart, HiSolidHeart } from "solid-icons/hi";

const Station = (props) => {

  const play = (station) => {
    invoke('play_station', { station: station });
    props.setCurrent(station);
  }

  const bookmark = (station) => {
    invoke('bookmark_station', { station: station });
  }

  return (
    <div class="relative group flex rounded-md shadow-lg shadow-black hover:bg-neutral-800">
      <img src={props.station.favicon} class="border-none" width={120} height={120} />
      <div class="ml-2 truncate flex flex-col">
        <span class="font-bold">{props.station.name}</span>
        <a target="_blank" href={props.station.homepage} class="truncate text-ellipsis text-slate-600 hover:text-slate-400 text-sm">{props.station.homepage}</a>
      </div>
      <button
        class="absolute top-5 left-5 opacity-0 h-fit w-fit group-hover:opacity-80 hover:!opacity-100 hover:!text-green-500"
        onClick={() => play(props.station)}>
        <HiSolidPlay class="h-20 w-20" />
      </button>
      <button
        class={`absolute right-0 -top-3 ${!props.station.bookmarked ? 'opacity-0 h-fit w-fit group-hover:opacity-100 group-hover:text-red-300 hover:!text-red-500' : 'text-red-500'}`}
        onClick={() => bookmark(props.station)}>
        {!props.station.bookmarked ? <HiOutlineHeart class="w-6 h-6" /> : <HiSolidHeart class="w-6 h-6" />}
      </button>
    </div>
  );
}

export default Station;
