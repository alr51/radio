import { createSignal, For, onMount } from "solid-js";
import { invoke } from "@tauri-apps/api/tauri";
import { appWindow } from "@tauri-apps/api/window";
import Station from "./station";
import Player from "./player";
import Search from "./Search";
import Header from "./Header";
import Loader from "./Loader";
import Visualizer from "./Visualizer";
import { Route, Routes } from "@solidjs/router";
import StationsList from "./StationsList";
import Favorites from "./Favorites";

async function invokeStreamEvents() {
  await invoke("stream_events", { window: appWindow });
}

invokeStreamEvents();


function App() {
  const [stations, setStations] = createSignal([]);
  const [currentStation, setCurrentStation] = createSignal(null);
  const [title, setTitle] = createSignal("");
  const [playing, setPlaying] = createSignal(false);
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
    setPlaying(true)
    setCurrentStation(station)
  }

  const play = () => {
    setPlaying(true)
    invoke('play')
  }

  const pause = () => {
    setPlaying(false)
    invoke('pause')
  }

  onMount(() => bookmarkList())

  return (
    <>
      <div class="mb-20 flex flex-col">
        <Loader pending={pending()} />
        <Header><Search search={searchStations} bookmarkList={bookmarkList} /></Header>
        <Routes>
          <Route path="/list" element={<StationsList stations={stations} setCurrent={setCurrent} />} />
          <Route path="/" element={<Favorites stations={stations} setCurrent={setCurrent} />} />
        </Routes>
      </div>
      <Visualizer />
      <Player currentStation={currentStation()} currentTitle={title()} playing={playing} pause={pause} play={play} />
    </>
  )
}

export default App;
