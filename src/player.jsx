import CurrentStation from "./CurrentStation"
import { invoke } from "@tauri-apps/api";
import { createSignal, Show } from "solid-js";
import { HiSolidPlay, HiSolidPause, HiOutlineVolumeOff, HiOutlineVolumeUp } from "solid-icons/hi"

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
        <div class="col-span-3 flex items-center">
          <span fex>{props.currentTitle}</span>
        </div>
        <div class="flex items-end ">
          <div class="inline-flex items-center space-x-1 mb-1">
          <HiOutlineVolumeOff />
          <input type="range" min="0" max="100" value="100" onChange={(e) => invoke("set_volume", { volume: e.target.value / 100 })} class="appearance-none bg-neutral-900 rounded-lg h-1 w-16"/>
          <HiOutlineVolumeUp />
    </div>
        </div>
      </Show>
    </div>
  )
}

export default Player
