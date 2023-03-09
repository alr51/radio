import { createSignal, For } from "solid-js";
import { invoke } from "@tauri-apps/api/tauri";
import { appWindow } from "@tauri-apps/api/window";
import Station from "./station";
import Player from "./player";
import Search from "./Search";

async function invokeStreamEvents() {
  await invoke("stream_events", { window: appWindow });
}

invokeStreamEvents();


function App() {
  const [stations, setStations] = createSignal([]);
  const [currentStation, setCurrentStation] = createSignal(undefined);
  const [title, setTitle] = createSignal("");

  appWindow.listen(
    'title_event',
    ({ payload }) => setTitle(payload)
  );

  async function searchStations(query) {
    setStations(await invoke("search_stations", { stationsQuery: { name: query.name, limit: query.limit } }));
  }

  const setCurrent = (station) => {
    setTitle("");
    setCurrentStation(station)
  }

  return (
    <>
      <div class="mb-20 flex flex-col">
        <Search search={searchStations} />
        <div class="grow grid grid-cols-2">
          <For each={stations()}>
            {(station) => <Station station={station} setCurrent={setCurrent} />}
          </For>
        </div>
      </div>
      <Player currentStation={currentStation()} currentTitle={title()} />
    </>
  );
}

export default App;
