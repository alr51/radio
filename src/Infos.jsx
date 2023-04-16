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

export default function Infos(props) {

  const artist = () => props.currentTitle.split(" - ")[0].trim()

  const [artistInfos] = createResource(artist, getArtistInfos)

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
        <Show when={artistInfos().bio}>
          <p>{artistInfos().bio}</p>
        </Show>
      </Show>
    </>
  )
}
