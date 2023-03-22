import { appWindow } from "@tauri-apps/api/window";
import { createSignal, onCleanup, onMount } from "solid-js";

function lerp (value1, value2, amount) {
	amount = amount < 0 ? 0 : amount;
	amount = amount > 1 ? 1 : amount;
	return value1 + (value2 - value1) * amount;
}

export default function Visualizer(props) {

  let spectrum = []
  let canvas

  appWindow.listen(
    'spectrum_event',
    ({ payload }) => spectrum = payload
  );

  onMount(() => {

    const ctx = canvas.getContext("2d")
    let frame
    function loop(t) {
      const canvasRect = canvas.getBoundingClientRect()
      const CANVAS_WIDTH = canvasRect.width
      const CANVAS_HEIGHT = canvasRect.height
      const BAR_WIDTH = canvasRect.width / 64

      ctx.canvas.width = CANVAS_WIDTH
      ctx.canvas.height = CANVAS_HEIGHT

      // ctx.clearRect(0, 0, CANVAS_WIDTH, CANVAS_HEIGHT)
      ctx.fillStyle = 'white'
      for (let i = 0; i < spectrum.length; i++) {
        const bar_y = lerp(0, CANVAS_HEIGHT, spectrum[i]/-80)
        ctx.fillRect(i * BAR_WIDTH, bar_y, BAR_WIDTH, CANVAS_HEIGHT - bar_y )
      }
      frame = requestAnimationFrame(loop)
    }

    loop()

    onCleanup(() => {
      cancelAnimationFrame(frame)
    })
  })

  return <div><canvas ref={canvas} class="border w-full h-32" /></div>
}
