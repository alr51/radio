import { invoke } from "@tauri-apps/api";
import { createResource, For, onCleanup, onMount, Show } from "solid-js";

const getArtistInfos = async (artist) => {
  let infos = { name: "", country: "", id: "", relations: [] }

  if (artist) {
    const info = await invoke("artist_info", { artist: artist })
    if (info) {
      infos = info
    }
  }

  return infos
}

const getArtistReleases = async (artistInfo) => {
  if (artistInfo) {
    const releases = await invoke("artist_releases", { artistid: artistInfo.artist.id })
    return releases
  }
  return
}

export default function Infos(props) {

  const artist = () => props.currentTitle.split(" - ")[0].trim()
  const [artistInfos] = createResource(artist, getArtistInfos)

  const [artistReleases] = createResource(artistInfos, getArtistReleases)

  let artistImages

  onMount(() => {
    let curImgIdx = 0
    const timer = setInterval(() => {
      const images = artistImages.getElementsByTagName('img')
      if (curImgIdx >= images.length) {
        curImgIdx = 0
      }
      if (images.length > 1) {
        for (let img of images) {
          img.classList.add('hidden')
        }
        images[curImgIdx].classList.toggle('hidden')
        curImgIdx++
      }
      return
    }, 10000)

    onCleanup(() => {
      clearInterval(timer)
    })
  })

  return (
    <>
      <Show when={!artistInfos.loading}>

        <div ref={artistImages} class="relative w-full min-h-[500px] bg-black">
          <Show when={artistInfos().images.artistbackground}>
            <For each={artistInfos().images.artistbackground}>
              {(image, idx) => <img src={image.url} class={`grayscale hover:grayscale-0 ${idx() > 0 ? 'hidden' : ''}`} />}
            </For>
          </Show>
          <div class="absolute w-full bottom-0 px-4 bg-black bg-opacity-70 text-7xl font-extrabold text-white">{artist()}</div>
        </div>

        <div class="p-4">
          <Show when={artistInfos().bio}>
            <h2 class="text-xl font-bold mb-2">Biography</h2>
            <p>{artistInfos().bio}</p>
          </Show>
          <Show when={!artistReleases.loading}>
            <h2 class="text-xl font-bold my-2">Discography ({artistReleases().count})</h2>
            <For each={artistReleases().releases}>
              {(release) => <ArtistRelease release={release} />}
            </For>
          </Show>
        </div>

      </Show>
    </>
  )
}

function ArtistRelease(props) {

  return <p><Show when={props.release.cover_art_archive.front} fallback={<div class="h-[120px] w-[120px] bg-blue-500"></div>}> <img src={`https://coverartarchive.org/release/${props.release.id}/front-250`} width="120" height="120" /></Show> {props.release.title} / {props.release.date} / {props.release.country} / {props.release.cover_art_archive.count} </p>

}
