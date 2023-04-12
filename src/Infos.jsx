import { invoke } from "@tauri-apps/api";
import { createResource, For, Show } from "solid-js";
import PageHeader from "./PageHeader";

const getArtistInfos = async (artist) => {
  let infos = { name: "", country: "", id: "", relations: [] }

  if (artist) {
    const info = await invoke("artist_info", { artist: artist })
    console.log(info)
    if (info) {
      infos = info
    }
  }

  return infos
}

export default function Infos(props) {

  const artist = () => props.currentTitle.split(" - ")[0].trim()

  const [artistInfos] = createResource(artist, getArtistInfos)


  return (
    <>
      <Show when={!artistInfos.loading}>
        <div class="relative">
          <Show when={artistInfos().images.artistbackground} fallback={<div class="w-full h-[500px] bg-black"></div>}>
            <img src={artistInfos().images.artistbackground[0].url} class="grayscale hover:grayscale-0" />
          </Show>
          <div class="absolute w-full bottom-0 px-4 bg-black bg-opacity-70 text-7xl font-extrabold text-white">{artist()}</div>
        </div>
    {/*<div>{artistInfos().artist.name} / {artistInfos().artist.id}</div>
        <ul>
          <For each={artistInfos().artist.relations}>
            {(relation) => <li><a href={relation.url.resource} target="_blank">{relation.url_type}</a></li>}
          </For>
        </ul> */}
      </Show>
    </>
  )
}



