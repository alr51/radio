import { createSignal, For, onMount } from "solid-js";
import { invoke } from "@tauri-apps/api/tauri";
import { appWindow } from "@tauri-apps/api/window";
import Station from "./station";
import Player from "./player";
import Search from "./Search";
import Header from "./Header";
import Loader from "./Loader";
import Visualizer from "./Visualizer";

async function invokeStreamEvents() {
  await invoke("stream_events", { window: appWindow });
}

invokeStreamEvents();


function App() {
  const [stations, setStations] = createSignal([]);
  const [currentStation, setCurrentStation] = createSignal(undefined);
  const [title, setTitle] = createSignal("");
  const [pending, setPending] = createSignal(false);

  appWindow.listen(
    'title_event',
    ({ payload }) => setTitle(payload)
  );

  async function searchStations(query) {
    setPending(true)
    const stations = await invoke("search_stations", { stationsQuery: { name: query.name, limit: query.limit } })
    setStations(stations)
    setPending(false)
  }

  async function bookmarkList() {
    setPending(true)
    const stations = await invoke("bookmark_stations_list")
    setStations(stations)
    setPending(false)
  }

  const setCurrent = (station) => {
    setTitle("");
    setCurrentStation(station)
  }

  onMount(() => bookmarkList())

  return (
    <>
      <div class="mb-20 flex flex-col">
        <Header><Search search={searchStations} bookmarkList={bookmarkList} /></Header>
        <Loader pending={pending()} />
        <div class="grow grid grid-cols-2 m-2 gap-4">
          <For each={stations()}>
            {(station) => <Station station={station} setCurrent={setCurrent} />}
          </For>
        </div>
      </div>
      <Visualizer />
      <Player currentStation={currentStation()} currentTitle={title()} />
    </>
  )
}

export default App;
