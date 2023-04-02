import { invoke } from "@tauri-apps/api";
import { HiSolidPlay, HiOutlineHeart, HiSolidHeart } from "solid-icons/hi";
import { createSignal, For, Show } from "solid-js";

const Station = (props) => {

  const [bookmarked, setBookmarked] = createSignal(props.station.bookmarked)

  const play = (station) => {
    invoke('play_station', { station: station });
    props.setCurrent(station);
  }

  const toggleBookmark = (station) => {
    bookmarked() ? invoke('remove_bookmark_station', { station: station }) : invoke('bookmark_station', { station: station })
    setBookmarked(!bookmarked())
  }

  const BookmarkButton = () => (
    <button
      class="absolute right-0 -top-3 opacity-0 h-fit w-fit group-hover:opacity-100 group-hover:text-red-300 hover:!text-red-500 group-hover:animate-bounce"
      onClick={() => toggleBookmark(props.station)}
    >
      <HiOutlineHeart class="w-6 h-6" />
    </button>
  )

  return (
    <div class="relative group flex rounded-md shadow-lg bg-neutral-800 bg-opacity-75 border border-neutral-800 shadow-black hover:border-black">
      <Show 
    when={props.station.favicon && props.station.favicon.length > 0} 
    fallback={<div class="h-[80px] w-[80px]"></div>}
    >
        <img src={props.station.favicon} class="rounded-tl-md rounded-bl-md min-h-[80px] min-w-[80px]" width={80} />
      </Show>
      <div class="ml-2 flex-grow truncate flex flex-col">
        <span class="font-bold">{props.station.name}</span>
        <a target="_blank" href={props.station.homepage} class="truncate text-ellipsis text-slate-600 hover:text-slate-400 text-sm">{props.station.homepage}</a>
      </div>
      <button
        class="absolute top-3 left-3 opacity-0 h-fit w-fit group-hover:opacity-100 hover:!text-green-500"
        onClick={() => play(props.station)}>
        <HiSolidPlay class="h-14 w-14" />
      </button>
      <Show
        when={bookmarked()}
        fallback={<BookmarkButton />}
      >
        <button class="absolute right-0 -top-3 text-red-500" onClick={() => toggleBookmark(props.station)}
        >
          <HiSolidHeart class="w-6 h-6" />
        </button>
      </Show>
    </div>
  );
}

export default Station;
