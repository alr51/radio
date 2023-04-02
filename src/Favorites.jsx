import PageHeader from "./PageHeader"
import Station from "./station"

export default function Favorites(props) {

  return (
    <>
      <PageHeader title="Favorites"/>
      <div class="grow grid grid-cols-2 m-4 gap-4">
        <For each={props.stations()}>
          {(station) => <Station station={station} setCurrent={props.setCurrent} />}
        </For>
      </div>
    </>)
}
