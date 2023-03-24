import CurrentStation from "./CurrentStation"
import { invoke } from "@tauri-apps/api";
import { createSignal, Show } from "solid-js";
import { HiSolidPlay, HiSolidPause } from "solid-icons/hi"

const Player = (props) => {

  const [playing, setPlaying] = createSignal(true);

  const play = () => {
    invoke('play')
  }

  const pause = () => {
    invoke('pause')
  }


  const toggle = () => {
    playing() ? pause() : play();
    setPlaying(!playing());
  }

  return (
    <div class="fixed bottom-0 h-20 w-full bg-black border-t border-neutral-800 grid grid-cols-9">
      <Show when={props.currentStation}>
        <CurrentStation station={props.currentStation} />
        <div class="flex items-center justify-center">
          <button onClick={() => toggle()} class="h-fit w-fit hover:text-white">
            {playing() ? <HiSolidPause class="h-16 w-16" /> : <HiSolidPlay class="h-16 w-16" />}
          </button>
        </div>
        <div class="col-span-4 flex items-center">{props.currentTitle}</div>
      </Show>
    </div>
  )
}

export default Player
