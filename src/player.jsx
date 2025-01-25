import CurrentStation from "./CurrentStation"
import { invoke } from "@tauri-apps/api/core";
import { Show } from "solid-js";
import { HiSolidPlay, HiSolidPause, HiOutlineVolumeOff, HiOutlineVolumeUp } from "solid-icons/hi"
import { A } from "@solidjs/router";

const Player = (props) => {

  const toggle = () => {
    props.playing() ? props.pause() : props.play();
  }

  const setVolume = (e) => {
    invoke("set_volume", { volume: e.target.value / 100 })
  }

  return (
    <div class="fixed bottom-0 h-20 w-full bg-black border-t border-neutral-800 grid grid-cols-9">
      <Show when={props.currentStation}>
        <CurrentStation station={props.currentStation} />
        <div class="flex items-center justify-center">
          <button onClick={() => toggle()} class="h-fit w-fit hover:text-white">
            {props.playing() ? <HiSolidPause class="h-16 w-16" /> : <HiSolidPlay class="h-16 w-16" />}
          </button>
        </div>
        <div class="col-span-3 flex items-center">
          <A href="/infos">{props.currentTitle}</A>
        </div>
        <div class="flex items-end ">
          <div class="inline-flex items-center space-x-1 mb-1">
            <HiOutlineVolumeOff />
            <input
              type="range"
              min="0"
              max="100"
              value="100"
              onChange={setVolume}
              class="appearance-none bg-neutral-900 rounded-lg h-1 w-16"
            />
            <HiOutlineVolumeUp />
          </div>
        </div>
      </Show>
    </div>
  )
}

export default Player
