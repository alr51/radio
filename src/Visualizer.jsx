import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
import { onCleanup, onMount } from "solid-js";
const appWindow = getCurrentWebviewWindow()

function lerp (value1, value2, amount) {
  amount = amount < 0 ? 0 : amount;
  amount = amount > 1 ? 1 : amount;
  return value1 + (value2 - value1) * amount;
}

export default function Visualizer() {

  let spectrum = []
  let canvas

  onMount(async () => {

    const unlisten = await appWindow.listen(
      'spectrum_event',
      ({ payload }) => spectrum = payload
    );

    const ctx = canvas.getContext("2d")
    let frame

    function loop() {
      
      const canvasRect = canvas.getBoundingClientRect()
      const CANVAS_WIDTH = canvasRect.width
      const CANVAS_HEIGHT = canvasRect.height
      const BAR_WIDTH = (canvasRect.width / spectrum.length) / 2

      ctx.canvas.width = CANVAS_WIDTH
      ctx.canvas.height = CANVAS_HEIGHT

      ctx.clearRect(0, 0, CANVAS_WIDTH, CANVAS_HEIGHT)
      for (let i = 0; i < spectrum.length; i++) {
        const bar_y = lerp(0, CANVAS_HEIGHT, spectrum[i]/-60)
	const r = 255 + spectrum[i] 
	const g = lerp(0,255, i/128)
	const b = lerp(255,0, i/128)
	ctx.fillStyle = `rgba(${r},${g},${b},1)`
        ctx.fillRect(i * BAR_WIDTH + CANVAS_WIDTH / 2, bar_y, BAR_WIDTH, CANVAS_HEIGHT - bar_y )
	ctx.fillRect((CANVAS_WIDTH /2) - (i * BAR_WIDTH) - BAR_WIDTH  , bar_y, BAR_WIDTH, CANVAS_HEIGHT - bar_y )
      }
      frame = requestAnimationFrame(loop)
    }

    loop()

    onCleanup(() => {
      unlisten()
      cancelAnimationFrame(frame)
    })
  })

  return <canvas ref={canvas} class="-z-10 fixed bottom-20 w-full h-32" />
}
