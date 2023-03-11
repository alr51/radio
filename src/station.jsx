import { invoke } from "@tauri-apps/api";

const Station = (props) => {

  const play = (station) => {
    invoke('play_station',{station:station});
    props.setCurrent(station);
  }

  const bookmark = (station) => {
    invoke('bookmark_station',{station:station});
  }
  
  return (
    <div class="flex space-x-4 ml-4 mb-6">
      <img src={props.station.favicon} width={50} />
      <div class="flex flex-col">
        <span class="font-bold">{props.station.name}</span>
        <a target="_blank" href={props.station.homepage} class="text-slate-600 hover:text-slate-400 text-sm">{props.station.homepage}</a>
      </div>
      <button onClick={() => play(props.station)}>Play</button>
      <button onClick={() => bookmark(props.station)}>Bookmark</button>
    </div>
  );
}

export default Station;
