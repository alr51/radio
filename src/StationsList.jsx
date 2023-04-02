import PageHeader from "./PageHeader";
import Station from "./station";

export default function StationsList(props) {

  return (
    <>
      <PageHeader title="Stations" />
      <div class="grow grid grid-cols-2 m-2 gap-4">
        <For each={props.stations()}>
          {(station) => <Station station={station} setCurrent={props.setCurrent} />}
        </For>
      </div>
    </>)
}
